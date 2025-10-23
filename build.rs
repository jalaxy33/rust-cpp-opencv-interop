fn main() {
    // -------------- Link External Libraries ----------------

    // find opencv
    let opencv_lib = vcpkg::find_package("opencv").map_err(|e| {
        println!("cargo:warning=OpenCV not found via vcpkg. Installing by 'vcpkg install opencv[contrib,nonfree,world]'...");
        std::process::Command::new("vcpkg")
            .args(["install", "opencv[contrib,nonfree,world]", "--classic"])
            .status()
            .expect("Failed to install OpenCV");
        e
    });

    // -------------- Build CXX Bridge Module ----------------

    let rust_sources = vec!["src/opencv_ffi.rs"];
    let cpp_sources = vec!["src/opencv_ffi.cpp", "src/example.cpp"];

    let mut build = cxx_build::bridges(rust_sources);
    build.files(cpp_sources).include("include").std("c++17");

    // Adjust opencv include paths when using vcpkg
    for include in opencv_lib.unwrap().include_paths {
        build.include(include.join("opencv4").to_str().unwrap());
    }

    build.compile("librust");
}
