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

db-up:
  docker compose up -d

db-down:
  docker compose down

db-reset:
  docker compose down -v
  docker compose up -d
