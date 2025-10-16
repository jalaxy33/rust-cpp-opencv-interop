cmake_minimum_required(VERSION 3.10.0)
project(rust-cpp-opencv-interop VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)


# =============================================================================
# Rust Library Configuration
# =============================================================================

set(RUST_LIB_NAME librust)   # replace `librust` with actual Rust library name
set(RUST_PROJECT_NAME rust-cpp-opencv-interop)  # replace `rust-to-cpp` with actual Cargo project name
set(RUST_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)

# Setup Rust library integration for target
function(setup_rust_library target_name)
    # Find Cargo executable
    find_program(CARGO_EXECUTABLE cargo REQUIRED)
    
    # Build Rust library in release mode during configuration
    message(NOTICE "Building Rust library in 'Release' mode...")
    execute_process(
        COMMAND cargo build --release
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    )
    
    # Define library paths (always use release)
    set(rust_lib_dir "${RUST_TARGET_DIR}/release")
    set(rust_lib_file "${CMAKE_STATIC_LIBRARY_PREFIX}${RUST_LIB_NAME}${CMAKE_SHARED_LIBRARY_SUFFIX}")
    
    # Fix Windows MSVC runtime library compatibility
    if(WIN32 AND MSVC)
        target_compile_options(${target_name} PRIVATE /MD)
    endif()
    
    # Setup CXX bridge include paths and sources
    target_include_directories(${target_name} PRIVATE 
        ${RUST_TARGET_DIR}/cxxbridge/rust
        ${RUST_TARGET_DIR}/cxxbridge/${RUST_PROJECT_NAME}/src
    )
    
    file(GLOB cxx_bridge_src "${RUST_TARGET_DIR}/cxxbridge/${RUST_PROJECT_NAME}/src/*.cc")
    if(cxx_bridge_src)
        target_sources(${target_name} PRIVATE ${cxx_bridge_src})
    endif()
        
    # Link Rust library
    set(rust_lib_suffix "$<IF:$<PLATFORM_ID:Windows>,.lib,>")
    target_link_libraries(${target_name} PRIVATE "${rust_lib_dir}/${rust_lib_file}${rust_lib_suffix}")
    
    # Link CXX runtime library
    file(GLOB cxx_dirs "${rust_lib_dir}/build/cxx-*/out")
    if(cxx_dirs)
        list(GET cxx_dirs 0 cxx_dir)
        file(GLOB cxx_lib "${cxx_dir}/*${CMAKE_STATIC_LIBRARY_SUFFIX}")
        if(cxx_lib)
            target_link_libraries(${target_name} PRIVATE ${cxx_lib})
        endif()
    else()
        message(WARNING "CXX library not found for release build")
    endif()
    
    # Copy DLL on Windows
    if(WIN32)
        add_custom_command(TARGET ${target_name} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
            "${rust_lib_dir}/${rust_lib_file}"
            $<TARGET_FILE_DIR:${target_name}>
            COMMENT "Copying Rust DLL for $<CONFIG>"
        )
    endif()
endfunction()


# =============================================================================
# Find Packages
# =============================================================================

set(OpenCV_DIR "${VCPKG_INSTALLED_DIR}/x64-windows/share/opencv4")

find_package(OpenCV REQUIRED)


# =============================================================================
# Target Configuration
# =============================================================================


set(targetname demo)
add_executable(${targetname} src/main.cpp src/example.cpp)
target_include_directories(${targetname} PRIVATE ${CMAKE_SOURCE_DIR}/include)

# link to outer libraries
setup_rust_library(${targetname})
target_link_libraries(${targetname} PRIVATE ${OpenCV_LIBS})

# Set PROJECT_ROOT as a global variable for C++ code
target_compile_definitions(${targetname} PRIVATE PROJECT_ROOT="${CMAKE_SOURCE_DIR}")

