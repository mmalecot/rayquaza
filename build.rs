use cmake::Config;
use fs_extra::{dir, dir::CopyOptions};
use std::{env, error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    // Computes paths
    let source_directory = env::var("CARGO_MANIFEST_DIR")?;
    let source_directory = Path::new(&source_directory);
    let destination_directory = env::var("OUT_DIR")?;
    let destination_directory = Path::new(&destination_directory);
    let raylib_source_directory = source_directory.join("external/raylib");
    let raylib_destination_directory = destination_directory.join("raylib");

    // Copies raylib to output directory.
    let options = CopyOptions::new();
    dir::copy(&raylib_source_directory, &destination_directory, &options)?;

    // Compiles raylib
    let mut config = Config::new(&raylib_destination_directory);
    let build_directory = config
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("SUPPORT_CAMERA_SYSTEM", "OFF")
        .define("SUPPORT_GESTURES_SYSTEM", "OFF")
        .define("SUPPORT_SCREEN_CAPTURE", "OFF")
        .define("SUPPORT_GIF_RECORDING", "OFF")
        .define("SUPPORT_COMPRESSION_API", "OFF")
        .define("SUPPORT_VR_SIMULATOR", "OFF")
        .define("SUPPORT_FILEFORMAT_FNT", "OFF")
        .define("SUPPORT_FILEFORMAT_TTF", "OFF")
        .define("SUPPORT_FILEFORMAT_OBJ", "OFF")
        .define("SUPPORT_FILEFORMAT_MTL", "OFF")
        .define("SUPPORT_FILEFORMAT_IQM", "OFF")
        .define("SUPPORT_FILEFORMAT_GLTF", "OFF")
        .define("SUPPORT_MESH_GENERATION", "OFF")
        .define("SUPPORT_FILEFORMAT_WAV", "OFF")
        .define("SUPPORT_FILEFORMAT_OGG", "OFF")
        .define("SUPPORT_FILEFORMAT_XM", "OFF")
        .define("SUPPORT_FILEFORMAT_MOD", "OFF")
        .define("SUPPORT_FILEFORMAT_FLAC", "OFF")
        .define("SUPPORT_FILEFORMAT_MP3", "OFF")
        .define("STATIC", "TRUE")
        .build();
    fs::remove_dir_all(&raylib_destination_directory)?;
    let library_directory = build_directory.join("lib");

    // Links libraries
    println!(
        "cargo:rustc-link-search=native={}",
        library_directory.display()
    );
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=static=raylib");
        println!("cargo:rustc-link-lib=X11");
    } else if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=raylib_static");
        println!("cargo:rustc-link-lib=dylib=winmm");
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=shell32");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=static=raylib");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    }
    Ok(())
}
