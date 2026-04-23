#[cfg(target_os = "windows")]
fn main() {
    use image::imageops::FilterType;
    use std::{env, path::PathBuf};

    println!("cargo:rerun-if-changed=icon.png");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let png_path = manifest_dir.join("icon.png");
    let ico_path = out_dir.join("icon.ico");

    let image = image::open(&png_path)
        .expect("failed to open icon.png")
        .resize(256, 256, FilterType::Lanczos3);
    image.save(&ico_path).expect("failed to generate icon.ico");

    let mut res = winresource::WindowsResource::new();
    res.set_icon(ico_path.to_str().expect("icon path should be valid UTF-8"));
    res.set("FileDescription", "SuperTable");
    res.set("ProductName", "SuperTable");
    res.compile().expect("failed to compile windows resources");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("cargo:rerun-if-changed=icon.png");
}
