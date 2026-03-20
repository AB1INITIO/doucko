#!/usr/bin/env bash
# watch.sh — rebuild _site/ whenever source files change (1 s poll, no deps)
set -euo pipefail

checksum() {
  {
    find mat inf ine -name "*.md" 2>/dev/null | sort | xargs cat 2>/dev/null
    cat style.css index.html pandoc-template.html 2>/dev/null
  } | cksum
}

echo "Watching for changes (Ctrl-C to stop)..."
last=$(checksum)

while true; do
  sleep 1
  current=$(checksum)
  if [ "$current" != "$last" ]; then
    last="$current"
    echo "[$(date +%H:%M:%S)] change detected — rebuilding..."
    bash scripts/build.sh
    echo "[$(date +%H:%M:%S)] done."
  fi
done
