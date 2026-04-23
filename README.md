# SuperTable

SuperTable is a fully open-source and free desktop database client prototype built with Rust,
`gpui`, and `gpui-component`. It currently focuses on a clean multi-panel interface for
connections, SQL editing, and result browsing, with no paid tier or paywalled features in the
current project.

## Features

- SuperTable desktop UI
- MIT licensed and fully open source
- No subscription, no pro tier, no paywall in the current project
- Built with `gpui` and `gpui-component`
- Modularized Rust source structure
- Custom application icon from `icon.png`
- Windows executable icon embedding via `build.rs`

## Project Structure

```text
src/
  assets.rs
  data.rs
  palette.rs
  main.rs
  ui/
    app.rs
    top_bar.rs
    sidebar.rs
    editor.rs
    results.rs
```

## Getting Started

### Prerequisites

- Rust 1.94 or newer
- A desktop environment supported by `gpui`
- On macOS, a full Xcode installation with the Metal toolchain available to `xcrun`

If `cargo build` fails with `cannot execute tool 'metal' due to missing Metal Toolchain`,
install the missing component and make sure the active developer directory points at Xcode:

```bash
xcodebuild -downloadComponent MetalToolchain
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

### Run

```bash
cargo run
```

Or with `just`:

```bash
just run
```

### Hot Reload Build

This project uses `watchexec` for the local edit-build-restart loop.

Install it once:

```bash
cargo install --locked watchexec-cli
```

Then start the watcher from the project root:

```powershell
./scripts/dev-watch.ps1
```

On macOS and Linux:

```bash
sh ./scripts/dev-watch.sh
```

Or use the cross-platform `just` entrypoint:

```bash
just watch
```

The watcher listens to:

- `src`
- `Cargo.toml`
- `Cargo.lock`
- `build.rs`
- `icon.png`

and ignores `target/**` so rebuild output does not trigger another restart.

### Build

```bash
cargo build
```

Or with `just`:

```bash
just build
```

### Task Runner

This repository includes a cross-platform [just](https://github.com/casey/just) task file.

Available commands:

```bash
just --list
```

## Notes

- The current implementation is a static UI prototype and does not execute real database queries yet.
- On Windows, `build.rs` converts `icon.png` into an `.ico` resource and embeds it into the executable.
- On macOS and Linux, `icon.png` is currently used inside the UI; platform-specific packaging icons can be added later.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
