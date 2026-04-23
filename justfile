default:
  @just --list

run:
  cargo run

build:
  cargo build

check:
  cargo check

clean:
  cargo clean

watch:
  {{ if os() == "windows" { "powershell -NoProfile -ExecutionPolicy Bypass -File ./scripts/dev-watch.ps1" } else { "sh ./scripts/dev-watch.sh" } }}
