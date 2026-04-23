#[cfg(target_os = "windows")]
fn main() {
    use image::imageops::FilterType;
    use std::{env, fs, path::PathBuf};

    println!("cargo:rerun-if-changed=icon.png");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let png_path = manifest_dir.join("icon.png");
    let ico_path = out_dir.join("icon.ico");
    let rc_path = out_dir.join("icon.rc");

    let image = image::open(&png_path)
        .expect("failed to open icon.png")
        .resize(256, 256, FilterType::Lanczos3);
    image.save(&ico_path).expect("failed to generate icon.ico");

    fs::write(&rc_path, format!("1 ICON \"{}\"\n", ico_path.display()))
        .expect("failed to write icon resource script");

    let _ = embed_resource::compile(rc_path, embed_resource::NONE);
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("cargo:rerun-if-changed=icon.png");
}
