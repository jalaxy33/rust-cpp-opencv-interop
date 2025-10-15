// build.rs

fn main() {
    let rust_sources = vec!["src/opencv_ffi.rs"];
    let cpp_sources = vec!["src/example.cpp"];

    let mut build = cxx_build::bridges(rust_sources);
    build
        .files(cpp_sources)
        .include("include")
        .std("c++17");

    if let Ok(opencv_include) = std::env::var("OPENCV_INCLUDE_PATHS") {
        build.include(&opencv_include);
    }

    build.compile("try-rust-opencv");

    println!("cargo:rerun-if-changed=src/example.cpp");
    println!("cargo:rerun-if-changed=include/example.h");
    println!("cargo:rerun-if-changed=src/opencv_ffi.rs");
}
