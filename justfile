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
  {{ if os() == "windows" { "powershell -NoProfile -ExecutionPolicy Bypass -File ./scripts/dev-watch.ps1" } else { "watchexec --restart --debounce 300ms --watch src --watch Cargo.toml --watch Cargo.lock --watch build.rs --watch icon.png --ignore target/** --ignore .git/** -- cargo run" } }}
