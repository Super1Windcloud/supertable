use std::path::Path;

pub const APP_ICON_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/icon.png");

pub fn app_icon_path() -> &'static Path {
    Path::new(APP_ICON_PATH)
}
