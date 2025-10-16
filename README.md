# Rust-C++ OpenCV FFI

This project demonstrates how to interoperate between Rust and C++ using OpenCV for image processing tasks. It includes examples of reading, resizing, and displaying images using both C++ and Rust functions.


## Prerequisites

- **C++ compiler**: g++/clang++/MSVC
- **[CMake](https://cmake.org/download/) build tools**
- **[Rust toolchain](https://www.rust-lang.org/tools/install)**
- **[vcpkg](https://vcpkg.io/en/index.html)**: C/C++ package manager.


## Usage

1. Configure the project with CMake using the provided presets in [`CMakePresets.json`](./CMakePresets.json):

   ```bash
   cmake --preset vcpkg   # For debug build
   cmake --preset vcpkg-release # For release build
   ```

2. Build the project:

    ```bash
    cmake --build build
    ```

3. Run the executable:

    ```bash
    ./build/demo.exe   # On Windows
    ./build/demo      # On Linux/macOS
    ```
