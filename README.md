# Rust-C++ OpenCV FFI

This project demonstrates how to interoperate between Rust and C++ using OpenCV for image processing tasks. It includes examples of reading, resizing, and displaying images using both C++ and Rust functions.


## Prerequisites

- **C++ compiler**: g++/clang++/MSVC
- **[CMake](https://cmake.org/download/) build tools**
- **[Rust toolchain](https://www.rust-lang.org/tools/install)**
- **[OpenCV](https://github.com/opencv/opencv/)**


## Usage

1. Set OpenCV-related environment variables following the instructions in [rust-opencv](https://github.com/twistedfall/opencv-rust?tab=readme-ov-file#environment-variables) in [`.cargo/config.toml`](./.cargo/config.toml). 

```toml
[env]

# for rust-opencv
# read: https://github.com/twistedfall/opencv-rust/tree/master?tab=readme-ov-file#environment-variables
OPENCV_LINK_LIBS = "opencv_world4120.lib,opencv_world4120d.lib"
OPENCV_LINK_PATHS = "D:/ScoopApps/apps/opencv/current/x64/vc16/lib"
OPENCV_INCLUDE_PATHS = "D:/ScoopApps/apps/opencv/current/include"
```

2. For Rust, build and run the project using Cargo:

```bash
# Build
cargo build  # Debug build
cargo build --release  # Release build

# run the executable
cargo run  # Debug
cargo run --release  # Release
```

3. For C++, build and run the project using CMake:

```bash
# Configure and build
cmake -B build && cmake --build build  # Debug build
cmake -B build && cmake --build build --config Release  # Release build

# Run the executable (change `demo` to actual target name)
./build/Debug/demo.exe  # On Windows (Debug mode)
./build/Debug/demo      # On Linux/Mac (Debug mode)
```


