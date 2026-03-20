#!/usr/bin/env bash
# build.sh — compile markdown sources into a static site under _site/
set -euo pipefail

SITE="${SITE:-_site}"

echo "Building into $SITE/ ..."

# ── Clean & prepare output ────────────────────────────────────────────────────
rm -rf "$SITE"
mkdir -p "$SITE"

# ── Static assets ─────────────────────────────────────────────────────────────
cp index.html "$SITE/index.html"
cp style.css  "$SITE/style.css"

# ── Process each section directory at repo root ───────────────────────────────
for section_dir in */; do
  [ -d "$section_dir" ] || continue
  section=$(basename "$section_dir")

  # Skip non-section directories
  case "$section" in
    _site|.git|scripts|.github|templates) continue ;;
  esac

  out_dir="$SITE/$section"
  mkdir -p "$out_dir"

  echo "  [$section]"

  # Convert each .md → .html
  shopt -s nullglob
  md_files=("$section_dir"*.md)
  shopt -u nullglob

  if [ ${#md_files[@]} -eq 0 ]; then
    echo "    (no markdown files found)"
  fi

  for mdfile in "${md_files[@]}"; do
    base=$(basename "$mdfile" .md)
    echo "    pandoc  $mdfile → $out_dir/$base.html"
    pandoc "$mdfile" \
      --template=pandoc-template.html \
      --standalone \
      --from=markdown+tex_math_dollars \
      --to=html5 \
      --katex \
      --highlight-style=haddock \
      --output="$out_dir/$base.html"
  done

  # Collect and sort generated html names (excluding index)
  names=()
  for f in "$out_dir"/*.html; do
    [ -f "$f" ] || continue
    base=$(basename "$f" .html)
    [ "$base" = "index" ] && continue
    names+=("$base")
  done
  IFS=$'\n' sorted_names=($(printf '%s\n' "${names[@]+"${names[@]}"}" | sort))
  unset IFS

  # Build the <li> block
  items=""
  for name in "${sorted_names[@]+"${sorted_names[@]}"}"; do
    items="${items}      <li><a href=\"${name}.html\">${name}</a></li>\n"
  done

  # Render section index from template
  echo "    index   → $out_dir/index.html"
  tpl=$(<index-template.html)
  tpl="${tpl//\{\{SECTION\}\}/$section}"
  tpl="${tpl//\{\{ITEMS\}\}/$(printf '%b' "$items")}"
  printf '%s\n' "$tpl" > "$out_dir/index.html"

done

echo "Done. Site written to $SITE/"
