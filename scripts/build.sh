#!/usr/bin/env bash
# build.sh — compile markdown sources into a static site under _site/
#
#   build.sh                   clean full build (CI, `make build`)
#   build.sh --update PATH...  incremental: refresh only the outputs that depend
#                              on the given source paths, which may have been
#                              added, changed or deleted (used by watch.sh)
#
# Each output depends on exactly one source, except a section's index.html
# (lists the section's pages) and the shared templates/scripts (used by every
# page). Files are written to a hidden temp file and renamed into place, so the
# server never sees a missing or half-written file.
set -euo pipefail

SITE="${SITE:-_site}"

# ── Helpers ───────────────────────────────────────────────────────────────────
# Failures are propagated with explicit `|| return 1`, because update mode calls
# these in a `||` context, where `set -e` is suspended.

# A section is any visible top-level directory that isn't tooling or output.
is_section() {
  case "$1" in
    ""|.*|_site|"$SITE"|scripts|templates) return 1 ;;
  esac
}

# Hidden sibling temp path for FILE, so the final rename is atomic.
tmp_for() { printf '%s/.%s.tmp' "${1%/*}" "${1##*/}"; }

# render_page SECTION/NAME.md — pandoc → $SITE/SECTION/NAME.html
render_page() {
  local md=$1 out tmp
  out="$SITE/${md%.md}.html"
  tmp=$(tmp_for "$out")
  echo "    pandoc  $md → $out"
  mkdir -p "${out%/*}" || return 1
  if python3 scripts/preprocess.py < "$md" \
     | pandoc \
       --template=pandoc-template.html \
       --standalone \
       --from=markdown+tex_math_dollars \
       --to=html5 \
       --katex \
       --highlight-style=haddock \
       --resource-path="$(dirname "$md")" \
       --output="$tmp"; then
    mv -f "$tmp" "$out"
  else
    rm -f "$tmp"
    return 1
  fi
}

# remove_output SRC OUT — delete OUT for a deleted SRC, plus any output dirs
# whose source dir is gone too (e.g. a deleted img/ folder or whole section).
remove_output() {
  local src=$1 out=$2 dir
  if [ -e "$out" ]; then
    echo "    remove  $out"
    rm -f "$out" || return 1
  fi
  dir=$(dirname "$src")
  while [ "$dir" != . ] && [ ! -d "$dir" ]; do
    rm -rf "${SITE:?}/$dir" || return 1
    dir=$(dirname "$dir")
  done
}

# sync_file SRC DST — copy SRC to DST, or remove DST if SRC no longer exists.
sync_file() {
  local src=$1 dst=$2 tmp
  if [ ! -f "$src" ]; then
    remove_output "$src" "$dst"
    return
  fi
  echo "    copy    $src → $dst"
  tmp=$(tmp_for "$dst")
  mkdir -p "${dst%/*}" && cp "$src" "$tmp" && mv -f "$tmp" "$dst" \
    || { rm -f "$tmp"; return 1; }
}

# render_index SECTION — list the section's generated pages in its index.html
render_index() {
  local section=$1 out_dir="$SITE/$1" f base items="" tpl tmp
  local names=()
  mkdir -p "$out_dir" || return 1

  for f in "$out_dir"/*.html; do
    [ -f "$f" ] || continue
    base=$(basename "$f" .html)
    [ "$base" = "index" ] && continue
    names+=("$base")
  done
  while IFS= read -r base; do
    if [ -n "$base" ]; then
      items="${items}      <li><a href=\"${base}.html\">${base}</a></li>\n"
    fi
  done < <(printf '%s\n' "${names[@]+"${names[@]}"}" | sort)

  echo "    index   → $out_dir/index.html"
  tpl=$(<index-template.html) || return 1
  tpl="${tpl//\{\{SECTION\}\}/$section}"
  tpl="${tpl//\{\{ITEMS\}\}/$(printf '%b' "$items")}"
  tmp=$(tmp_for "$out_dir/index.html")
  printf '%s\n' "$tpl" > "$tmp" && mv -f "$tmp" "$out_dir/index.html" \
    || { rm -f "$tmp"; return 1; }
}

# build_section SECTION — render every page, copy assets, render the index
build_section() {
  local section=$1 md item
  local md_files=()
  echo "  [$section]"
  mkdir -p "$SITE/$section"

  shopt -s nullglob
  md_files=("$section"/*.md)
  shopt -u nullglob
  if [ ${#md_files[@]} -eq 0 ]; then
    echo "    (no markdown files found)"
  fi
  for md in "${md_files[@]+"${md_files[@]}"}"; do
    render_page "$md"
  done

  # Copy static assets (images, etc.) — anything that isn't a .md file
  shopt -s nullglob
  for item in "$section"/*; do
    [[ "$item" == *.md ]] && continue
    cp -r "$item" "$SITE/$section/"
  done
  shopt -u nullglob

  render_index "$section"
}

build_all() {
  local d
  sync_file index.html "$SITE/index.html"
  sync_file style.css  "$SITE/style.css"
  for d in */; do
    if is_section "${d%/}"; then
      build_section "${d%/}"
    fi
  done
}

# update PATH... — refresh only the outputs that depend on the given paths.
# A failing file doesn't stop the others; the exit status reports it.
update() {
  local p section rest out failed=0 all_indexes=0
  local index_sections=()

  # Shared inputs: every page depends on these.
  for p in "$@"; do
    case "$p" in
      pandoc-template.html|scripts/preprocess.py|scripts/build.sh)
        echo "  $p changed — rebuilding every page"
        build_all
        return ;;
    esac
  done

  for p in "$@"; do
    case "$p" in
      index.html|style.css)
        sync_file "$p" "$SITE/$p" || failed=1 ;;
      index-template.html)
        all_indexes=1 ;;
      */*)
        section=${p%%/*}
        rest=${p#*/}
        is_section "$section" || continue
        if [[ "$rest" == *.md && "$rest" != */* ]]; then
          # A page. The index lists pages, so it changes only when one is
          # added or removed, not when one is edited.
          out="$SITE/${p%.md}.html"
          if [ -f "$p" ]; then
            if [ ! -f "$out" ]; then index_sections+=("$section"); fi
            render_page "$p" || failed=1
          else
            remove_output "$p" "$out" || failed=1
            index_sections+=("$section")
          fi
        else
          # An asset. A top-level .html asset also shows up in the index.
          sync_file "$p" "$SITE/$p" || failed=1
          if [[ "$rest" == *.html && "$rest" != */* ]]; then
            index_sections+=("$section")
          fi
        fi
        if [ -d "$section" ] && [ ! -f "$SITE/$section/index.html" ]; then
          index_sections+=("$section")   # brand-new section
        fi ;;
      # Anything else (Makefile, README, …) isn't part of the site.
    esac
  done

  if (( all_indexes )); then
    index_sections=()
    for p in */; do
      if is_section "${p%/}"; then index_sections+=("${p%/}"); fi
    done
  fi
  while IFS= read -r section; do
    if [ -n "$section" ] && [ -d "$section" ]; then
      render_index "$section" || failed=1
    fi
  done < <(printf '%s\n' "${index_sections[@]+"${index_sections[@]}"}" | sort -u)

  return "$failed"
}

# ── Main ──────────────────────────────────────────────────────────────────────
if [ "${1:-}" = "--update" ]; then
  shift
  update "$@"
  exit
fi

echo "Building into $SITE/ ..."
rm -rf "$SITE"
mkdir -p "$SITE"
build_all
echo "Done. Site written to $SITE/"
