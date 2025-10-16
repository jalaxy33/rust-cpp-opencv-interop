// build.rs

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Check and install OpenCV via vcpkg if not present
    setup_opencv();

    // Use cxx crate to build C++ code and generate Rust bindings
    let rust_sources = vec!["src/opencv_ffi.rs"];
    let cpp_sources = vec!["src/example.cpp"];

    let mut build = cxx_build::bridges(rust_sources);
    build.files(cpp_sources).include("include").std("c++17");

    match std::env::var("OPENCV_INCLUDE_PATHS") {
        Ok(opencv_include) => {
            build.include(&opencv_include);
        }
        Err(_) => {
            panic!("OPENCV_INCLUDE_PATHS environment variable not found");
        }
    }

    build.compile("try-rust-opencv");

    println!("cargo:rerun-if-changed=src/example.cpp");
    println!("cargo:rerun-if-changed=include/example.h");
    println!("cargo:rerun-if-changed=src/opencv_ffi.rs");
}

fn setup_opencv() {
    // Determine platform-specific paths and library names
    let (triplet, lib_name) = if cfg!(target_os = "windows") {
        ("x64-windows", "opencv_core4.lib")
    } else if cfg!(target_os = "linux") {
        ("x64-linux", "libopencv_core4.a")
    } else if cfg!(target_os = "macos") {
        ("x64-osx", "libopencv_core4.a")
    } else {
        panic!("Unsupported platform");
    };

    let vcpkg_root = env::var("VCPKG_ROOT").unwrap();
    let opencv_dir = Path::new(&vcpkg_root).join("installed").join(triplet);
    let opencv_lib = opencv_dir.join("lib").join(lib_name);

    // Install OpenCV if library not found
    if !opencv_lib.exists() {
        println!("cargo:warning=OpenCV not found, checking vcpkg...");

        // Verify vcpkg is available
        if Command::new("vcpkg").arg("version").output().is_err() {
            println!("cargo:warning=vcpkg not found. Please install vcpkg first");
            panic!("vcpkg unavailable");
        }

        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        println!("cargo:warning=Installing OpenCV via vcpkg...");
        let result = Command::new("vcpkg")
            .args(&["install", "opencv4[contrib,nonfree]", "--classic"])
            .current_dir(&manifest_dir)
            .status()
            .expect("Failed to execute vcpkg install");

        if !result.success() {
            panic!("vcpkg install failed");
        }
    }

    // Set environment variables for OpenCV paths
    let opencv_include = opencv_dir.join("include").join("opencv4");
    unsafe {
        env::set_var("OPENCV_INCLUDE_PATHS", opencv_include.to_str().unwrap());
    }
}
