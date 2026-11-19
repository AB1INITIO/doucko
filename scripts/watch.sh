#!/usr/bin/env bash
# watch.sh — when source files change, rebuild only the affected outputs in
# _site/ (1 s poll, no deps). Changes are found by diffing a snapshot of every
# source file's mtime and size, so polling never reads file contents.
set -euo pipefail

SITE="${SITE:-_site}"

# One "<mtime-ns> <size> <path>" line per file, for GNU or BSD (macOS) stat.
if stat --version >/dev/null 2>&1; then
  STAT=(stat -c '%.9Y %s %n')
else
  STAT=(stat -f '%Fm %z %N')
fi

# Snapshot every source file. Hidden entries and the output dir are skipped, as
# build.sh skips them. NUL-delimited so names with spaces survive. A file that
# vanishes mid-scan (editor temp files) is ignored; the next poll settles it.
snapshot() {
  { find . \( -path './.*' -o -path "./$SITE" \) -prune -o -type f ! -name '.*' -print0 \
      | xargs -0 "${STAT[@]}" 2>/dev/null || true; } | LC_ALL=C sort
}

# changed_paths OLD NEW — paths whose line differs: added, modified or deleted.
changed_paths() {
  LC_ALL=C comm -3 <(printf '%s\n' "$1") <(printf '%s\n' "$2") \
    | sed $'s/^\t//' | cut -d' ' -f3- | sed 's|^\./||' | LC_ALL=C sort -u
}

# Microseconds since epoch (bash 5+), for reporting how long an update took.
now_us() { local t=${EPOCHREALTIME:-}; echo "${t/[.,]/}"; }

echo "Watching for changes (Ctrl-C to stop)..."
last=$(snapshot)

while true; do
  sleep 1
  current=$(snapshot)
  if [ "$current" = "$last" ]; then continue; fi

  changed=()
  while IFS= read -r p; do
    if [ -n "$p" ]; then changed+=("$p"); fi
  done < <(changed_paths "$last" "$current")
  last=$current
  if [ ${#changed[@]} -eq 0 ]; then continue; fi

  echo "[$(date +%H:%M:%S)] changed: $(printf '"%s" ' "${changed[@]}")"
  t0=$(now_us)
  if bash scripts/build.sh --update "${changed[@]}"; then
    t1=$(now_us)
    took=""
    if [ -n "$t0" ]; then took=" in $(( (t1 - t0) / 1000 )) ms"; fi
    echo "[$(date +%H:%M:%S)] updated${took}."
  else
    echo "[$(date +%H:%M:%S)] update FAILED (see above) — the rest of the site is still served." >&2
  fi
done
