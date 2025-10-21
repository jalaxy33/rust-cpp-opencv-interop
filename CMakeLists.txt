# =============================================================================
# Preamble
# =============================================================================

cmake_minimum_required(VERSION 3.10.0)

# vcpkg integration
set(CMAKE_TOOLCHAIN_FILE "$ENV{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake")

# =============================================================================
# Project Configuration
# =============================================================================

project(rust-cpp-opencv VERSION 0.1.0 LANGUAGES C CXX)
set(CMAKE_CXX_STANDARD 17)

# =============================================================================
# Rust Configuration
# =============================================================================

function(setup_rust_library targetname)
    # Fix Windows MSVC runtime library compatibility
    if(WIN32 AND MSVC)
        target_compile_options(${targetname} PRIVATE /MD)
    endif()

    # Ensure the Rust library is built
    set(RUST_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)
    set(rust_lib_dir ${RUST_TARGET_DIR}/release)   # assuming release build

    if(NOT EXISTS ${rust_lib_dir})
        message(FATAL_ERROR "Rust library not found at ${rust_lib_dir}. Please build the Rust library first with 'cargo build --release'.")
    endif()

    # Search for the Rust library
    file(GLOB rust_lib_files
        "${rust_lib_dir}/${CMAKE_STATIC_LIBRARY_PREFIX}*${CMAKE_SHARED_LIBRARY_SUFFIX}"
    )

    # link Rust library
    foreach(rust_lib IN LISTS rust_lib_files)
        if (WIN32)
            set(rust_lib "${rust_lib}.lib")
        endif()
        target_link_libraries(${targetname} PRIVATE ${rust_lib})
    endforeach()

    
    # Set include directories for cxx generated headers
    file(GLOB cxxbridge_include_dirs
        "${RUST_TARGET_DIR}/cxxbridge/*/src"
    )
    target_include_directories(${targetname} PRIVATE
        ${RUST_TARGET_DIR}/cxxbridge
        ${RUST_TARGET_DIR}/cxxbridge/rust
        ${cxxbridge_include_dirs}
    )

    # Gather cxxbridge generated source files
    file(GLOB cxxbridge_src
        "${RUST_TARGET_DIR}/cxxbridge/*/src/*.rs.cpp"
        "${RUST_TARGET_DIR}/cxxbridge/*/src/*.rs.cc"
    )

    if (cxxbridge_src)
        target_sources(${targetname} PRIVATE ${cxxbridge_src})
    endif()

    # link cxx library
    file(GLOB cxx_lib_files
        "${rust_lib_dir}/build/cxx-*/out/*${CMAKE_STATIC_LIBRARY_SUFFIX}"
    )
    foreach(cxx_lib IN LISTS cxx_lib_files)
        target_link_libraries(${targetname} PRIVATE ${cxx_lib})
    endforeach()

    # Copy DLL on Windows
    if(WIN32)
        add_custom_command(TARGET ${targetname} POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy_if_different
            "${rust_lib_files}"
            $<TARGET_FILE_DIR:${targetname}>
            COMMENT "Copying Rust DLL for $<CONFIG>"
        )
    endif()

endfunction()



# =============================================================================
# Find Packages
# =============================================================================

find_package(OpenCV REQUIRED)
link_libraries(${OpenCV_LIBS})


# =============================================================================
# Target Configuration
# =============================================================================

set(targetname demo)
add_executable(${targetname} src/main.cpp src/example.cpp src/opencv_ffi.cpp)
target_include_directories(${targetname} PRIVATE ${CMAKE_SOURCE_DIR}/include)

setup_rust_library(${targetname})

# Set global definitions for c++ codes
target_compile_definitions(${targetname} PRIVATE PROJECT_ROOT="${CMAKE_SOURCE_DIR}")



