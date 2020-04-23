use cmake::Config;
use flate2::read::GzDecoder;
use std::{
    env,
    error::Error,
    fs::{self, File},
    io,
    path::Path,
};
use tar::Archive;

const GITHUB_DOWNLOAD_URL: &str = "https://codeload.github.com";
const RAYLIB_REPOSITORY_OWNER: &str = "mmalecot";
const RAYLIB_REPOSITORY_NAME: &str = "raylib";
const RAYLIB_ARCHIVE_EXTENSION: &str = "tar.gz";
const RAYLIB_VERSION: &str = "3.0.0";

fn main() -> Result<(), Box<dyn Error>> {
    // Computes paths
    let out_directory = env::var("OUT_DIR")?;
    let out_directory = Path::new(&out_directory);
    let raylib_archive_url = format!(
        "{url}/{owner}/{repository}/{extension}/{reference}",
        url = GITHUB_DOWNLOAD_URL,
        owner = RAYLIB_REPOSITORY_OWNER,
        repository = RAYLIB_REPOSITORY_NAME,
        extension = RAYLIB_ARCHIVE_EXTENSION,
        reference = RAYLIB_VERSION,
    );
    let raylib_archive_file = format!(
        "{name}-{version}.{extension}",
        name = RAYLIB_REPOSITORY_NAME,
        version = RAYLIB_VERSION,
        extension = RAYLIB_ARCHIVE_EXTENSION
    );
    let raylib_archive_file = out_directory.join(&raylib_archive_file);
    let raylib_directory = format!(
        "{name}-{version}",
        name = RAYLIB_REPOSITORY_NAME,
        version = RAYLIB_VERSION,
    );
    let raylib_directory = out_directory.join(&raylib_directory);

    if !raylib_directory.exists() {
        // Downloads raylib
        let response = ureq::get(&raylib_archive_url).call();
        let mut reader = response.into_reader();
        let mut writer = File::create(&raylib_archive_file)?;
        io::copy(&mut reader, &mut writer)?;

        // Decompresses archive
        let file = File::open(&raylib_archive_file)?;
        let reader = GzDecoder::new(file);
        let mut archive = Archive::new(reader);
        archive.unpack(&out_directory)?;
        fs::remove_file(&raylib_archive_file)?;
    }

    // Compiles raylib
    let mut config = Config::new(&raylib_directory);
    let build_directory = config
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_GAMES", "OFF")
        .define("SUPPORT_CAMERA_SYSTEM", "OFF")
        .define("SUPPORT_GESTURES_SYSTEM", "OFF")
        .define("SUPPORT_MOUSE_GESTURES", "OFF")
        .define("SUPPORT_SSH_KEYBOARD_RPI", "OFF")
        .define("SUPPORT_MOUSE_CURSOR_RPI", "OFF")
        .define("SUPPORT_SCREEN_CAPTURE", "OFF")
        .define("SUPPORT_GIF_RECORDING", "OFF")
        .define("SUPPORT_COMPRESSION_API", "OFF")
        .define("SUPPORT_DATA_STORAGE", "OFF")
        .define("SUPPORT_VR_SIMULATOR", "OFF")
        .define("SUPPORT_FILEFORMAT_PNG", "ON")
        .define("SUPPORT_FILEFORMAT_BMP", "ON")
        .define("SUPPORT_FILEFORMAT_TGA", "ON")
        .define("SUPPORT_FILEFORMAT_JPG", "ON")
        .define("SUPPORT_FILEFORMAT_GIF", "ON")
        .define("SUPPORT_FILEFORMAT_PSD", "ON")
        .define("SUPPORT_FILEFORMAT_DDS", "ON")
        .define("SUPPORT_FILEFORMAT_HDR", "ON")
        .define("SUPPORT_FILEFORMAT_KTX", "ON")
        .define("SUPPORT_FILEFORMAT_ASTC", "ON")
        .define("SUPPORT_FILEFORMAT_PKM", "ON")
        .define("SUPPORT_FILEFORMAT_PVR", "ON")
        .define("SUPPORT_IMAGE_EXPORT", "OFF")
        .define("SUPPORT_IMAGE_MANIPULATION", "OFF")
        .define("SUPPORT_IMAGE_GENERATION", "OFF")
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
        .define("SUPPORT_TRACELOG", "OFF")
        .define("STATIC", "TRUE")
        .build();
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
