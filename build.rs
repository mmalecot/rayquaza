#[cfg(feature = "static")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Imports items
    use cmake::Config;
    use flate2::read::GzDecoder;
    use std::{
        env,
        fs::{self, File},
        io,
        path::Path,
    };
    use tar::Archive;

    // Defines constants
    const RAYLIB_NAME: &str = "raylib";
    const RAYLIB_ARCHIVE_EXTENSION: &str = "tar.gz";
    const RAYLIB_VERSION: &str = "3.0.0";
    const RAYLIB_GITHUB_REPOSITORY: &str = "raysan5/raylib";
    const GITHUB_CODELOAD_URL: &str = "https://codeload.github.com";

    // Computes paths
    let out_directory = env::var("OUT_DIR")?;
    let out_directory = Path::new(&out_directory);
    let raylib_archive_url = format!(
        "{url}/{repository}/{extension}/{version}",
        url = GITHUB_CODELOAD_URL,
        repository = RAYLIB_GITHUB_REPOSITORY,
        extension = RAYLIB_ARCHIVE_EXTENSION,
        version = RAYLIB_VERSION,
    );
    let raylib_archive_file = format!(
        "{name}-{version}.{extension}",
        name = RAYLIB_NAME,
        version = RAYLIB_VERSION,
        extension = RAYLIB_ARCHIVE_EXTENSION
    );
    let raylib_archive_file = out_directory.join(&raylib_archive_file);
    let raylib_directory = format!(
        "{name}-{version}",
        name = RAYLIB_NAME,
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
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_GAMES", "OFF")
        .define("SUPPORT_SCREEN_CAPTURE", "OFF")
        .define("SUPPORT_GIF_RECORDING", "OFF")
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

#[cfg(not(feature = "static"))]
fn main() {
    // Links dynamically raylib
    println!("cargo:rustc-link-lib=raylib");
}
