# SuperTable

SuperTable is a desktop database client prototype built with Rust, `gpui`, and `gpui-component`.
It currently focuses on a TablePlus-inspired interface with a clean multi-panel layout for connections,
SQL editing, and result browsing.

## Features

- TablePlus-inspired desktop UI
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

### Run

```bash
cargo run
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

## Notes

- The current implementation is a static UI prototype and does not execute real database queries yet.
- On Windows, `build.rs` converts `icon.png` into an `.ico` resource and embeds it into the executable.
- On macOS and Linux, `icon.png` is currently used inside the UI; platform-specific packaging icons can be added later.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
