# Rust-C++ OpenCV Interoperability demo

This project demonstrates how to interoperate between Rust and C++ using OpenCV for image processing tasks. It includes examples of reading, resizing, and displaying images using both C++ and Rust functions.


## Prerequisites

- **C++ compiler**: g++/clang++/MSVC
- **[CMake](https://cmake.org/download/) build tools**
- **[Rust toolchain](https://www.rust-lang.org/tools/install)**
- **[vcpkg](https://vcpkg.io/en/index.html)**: C/C++ package manager.


## Usage

> **Note**: Current implementation only supports **RELEASE** build, and it is only tested on Windows with MSVC toolchain.


1. Build the Rust library in release mode:

   ```bash
   cargo build --release   
   # or 
   cargo b -r
   ```

2. Configure and build the C++ project using CMake:

   ```bash
   cmake -B build && cmake --build build --config Release
   ```

3. Run the resulting C++ executable:

   ```bash
   ./build/release/demo.exe  # On Windows  (release build)
    ./build/release/demo  # On Linux/macOS (release build)
   ```

4. Run the Rust binary:

   ```bash
   cargo run   # debug build (or `cargo r`)
   cargo run --release  # release build (or `cargo r -r`)
   ```

   Run all Rust tests:

   ```bash
   cargo test  # debug build (or `cargo t`)
   cargo test --release  # release build (or `cargo t -r`)
   ```

