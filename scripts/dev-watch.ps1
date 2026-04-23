$ErrorActionPreference = "Stop"

if (-not (Get-Command watchexec -ErrorAction SilentlyContinue)) {
    Write-Error @"
watchexec is not installed.

Install it with:
  cargo install --locked watchexec-cli

Then run:
  ./scripts/dev-watch.ps1
"@
}

$arguments = @(
    "--restart"
    "--debounce"
    "300ms"
    "--watch"
    "src"
    "--watch"
    "Cargo.toml"
    "--watch"
    "Cargo.lock"
    "--watch"
    "build.rs"
    "--watch"
    "icon.png"
    "--ignore"
    "target/**"
    "--ignore"
    ".git/**"
    "--"
    "cargo"
    "run"
)

& watchexec @arguments
