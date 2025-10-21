fn main() {
    let opencv_lib = vcpkg::Config::new()
        .emit_includes(true)
        .find_package("opencv")
        .expect("Could not find OpenCV via vcpkg");

    let rust_sources = vec!["src/opencv_ffi.rs"];
    let cpp_sources = vec!["src/opencv_ffi.cpp", "src/example.cpp"];
    let mut build = cxx_build::bridges(rust_sources);
    build
        .files(cpp_sources)
        .include("include")
        .std("c++17");

    // Adjust opencv include paths when using vcpkg
    for include in opencv_lib.include_paths {
        build.include(include.join("opencv4").to_str().unwrap());
    }

    build.compile("librust");
}
