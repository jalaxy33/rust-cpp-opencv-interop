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
# Opencv Helper functions
# =============================================================================

function(read_toml toml_file)
    if (NOT EXISTS ${toml_file})
        message(FATAL_ERROR "TOML file ${toml_file} does not exist.")
    endif()

    file(STRINGS ${toml_file} toml_content)
    set(current_section "")
    
    foreach(line ${toml_content})
        # Remove leading/trailing whitespace
        string(STRIP "${line}" line)
        
        # Skip comments and empty lines
        if(NOT line MATCHES "^#" AND NOT line STREQUAL "")
            # Check if it's a section header [section_name]
            if(line MATCHES "^\\[([^\\]]+)\\]")
                set(current_section "${CMAKE_MATCH_1}")
                message(STATUS "Parsing TOML section: [${current_section}]")
            # Check for key = "value" format
            elseif(line MATCHES "^([A-Za-z_][A-Za-z0-9_]*)[ ]*=[ ]*\"([^\"]+)\"")
                set(key_name "${CMAKE_MATCH_1}")
                set(key_value "${CMAKE_MATCH_2}")
                
                # Create variable name with section prefix if we're in a section
                if(NOT current_section STREQUAL "")
                    string(TOUPPER "${current_section}_${key_name}" var_name)
                else()
                    string(TOUPPER "${key_name}" var_name)
                endif()
                
                set(${var_name} "${key_value}" CACHE STRING "From TOML file" FORCE)
                message(STATUS "Loaded TOML ${var_name} = ${key_value}")
            # Check for key = value format (without quotes)
            elseif(line MATCHES "^([A-Za-z_][A-Za-z0-9_]*)[ ]*=[ ]*([^\"#]+)")
                set(key_name "${CMAKE_MATCH_1}")
                set(key_value "${CMAKE_MATCH_2}")
                
                # Remove trailing whitespace and comments
                string(STRIP "${key_value}" key_value)
                string(REGEX REPLACE "[ ]*#.*$" "" key_value "${key_value}")
                string(STRIP "${key_value}" key_value)
                
                # Create variable name with section prefix if we're in a section
                if(NOT current_section STREQUAL "")
                    string(TOUPPER "${current_section}_${key_name}" var_name)
                else()
                    string(TOUPPER "${key_name}" var_name)
                endif()
                
                set(${var_name} "${key_value}" CACHE STRING "From TOML file" FORCE)
                message(STATUS "Loaded TOML ${var_name} = ${key_value}")
            endif()
        endif()
    endforeach()
endfunction()


function(check_var_defined var_name)
    if(NOT DEFINED ${var_name} OR ${var_name} STREQUAL "")
        message(FATAL_ERROR "${var_name} is not set. Please define it in the .env file or as a CMake variable.")
    endif()
endfunction()


function(setup_opencv targetname)
    # Try to read TOML config file first, fallback to .env file
    if(EXISTS "${CMAKE_SOURCE_DIR}/.cargo/config.toml")
        message(STATUS "Reading OpenCV configuration from config.toml")
        read_toml("${CMAKE_SOURCE_DIR}/.cargo/config.toml")

        # TOML variables will be prefixed with section names (OPENCV_*)
        set(OPENCV_INCLUDE_PATHS ${ENV_OPENCV_INCLUDE_PATHS})
        set(OPENCV_LINK_PATHS ${ENV_OPENCV_LINK_PATHS})
        set(OPENCV_LINK_LIBS ${ENV_OPENCV_LINK_LIBS})
    else()
        message(FATAL_ERROR "Neither config.toml nor .env file found. Please create one with OpenCV configuration.")
    endif()

    # check required variables
    check_var_defined(OPENCV_INCLUDE_PATHS)
    check_var_defined(OPENCV_LINK_PATHS)
    check_var_defined(OPENCV_LINK_LIBS)

    # link to library
    target_include_directories(${targetname} PRIVATE ${OPENCV_INCLUDE_PATHS})
    target_link_directories(${targetname} PRIVATE ${OPENCV_LINK_PATHS})

    string(REPLACE "," ";" OPENCV_LIBS_LIST ${OPENCV_LINK_LIBS})
    
    # Always use only release libraries for all configurations
    # This simplifies the build and avoids debug/release library compatibility issues
    set(opencv_libs "")
    
    foreach(lib ${OPENCV_LIBS_LIST})
        string(STRIP "${lib}" lib_clean)
        
        # Only include release libraries (skip those ending with 'd')
        if(NOT (lib_clean MATCHES "d\\.(lib|so|dylib)$" OR lib_clean MATCHES "d$"))
            list(APPEND opencv_libs ${lib_clean})
        endif()
    endforeach()
    
    if(opencv_libs)
        # Link release OpenCV libraries for all build configurations
        target_link_libraries(${targetname} PRIVATE ${opencv_libs})
        message(STATUS "Linking OpenCV release libraries for all configurations: ${opencv_libs}")
    else()
        message(FATAL_ERROR "No OpenCV release libraries found in OPENCV_LINK_LIBS!")
    endif()

endfunction()


# =============================================================================
# Target Configuration
# =============================================================================

# Example: Read project configuration from TOML
if(EXISTS "${CMAKE_SOURCE_DIR}/config.toml")
    read_toml("${CMAKE_SOURCE_DIR}/config.toml")
    
    # Use TOML values if available
    if(DEFINED BUILD_CXX_STANDARD)
        set(CMAKE_CXX_STANDARD ${BUILD_CXX_STANDARD})
        message(STATUS "Set C++ standard to ${CMAKE_CXX_STANDARD} from TOML config")
    endif()
    
    if(DEFINED PROJECT_NAME AND DEFINED PROJECT_VERSION)
        message(STATUS "Project info from TOML: ${PROJECT_NAME} v${PROJECT_VERSION}")
    endif()
endif()

set(targetname demo)
add_executable(${targetname} src/main.cpp src/example.cpp)
target_include_directories(${targetname} PRIVATE ${CMAKE_SOURCE_DIR}/include)

# link to outer libraries
setup_rust_library(${targetname})
setup_opencv(${targetname})

# Set PROJECT_ROOT as a global variable for C++ code
target_compile_definitions(${targetname} PRIVATE PROJECT_ROOT="${CMAKE_SOURCE_DIR}")


