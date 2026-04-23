#!/usr/bin/env sh
set -eu

if ! command -v watchexec >/dev/null 2>&1; then
  cat <<'EOF' >&2
watchexec is not installed.

Install it with:
  cargo install --locked watchexec-cli

Then run:
  ./scripts/dev-watch.sh
EOF
  exit 1
fi

exec watchexec \
  --restart \
  --debounce 300ms \
  --watch src \
  --watch Cargo.toml \
  --watch Cargo.lock \
  --watch build.rs \
  --watch icon.png \
  --ignore 'target/**' \
  --ignore '.git/**' \
  -- \
  cargo run
