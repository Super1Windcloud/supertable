use std::path::Path;

#[allow(dead_code)]
pub const APP_ICON_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/icon.png");

#[allow(dead_code)]
pub fn app_icon_path() -> &'static Path {
    Path::new(APP_ICON_PATH)
}
