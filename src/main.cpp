#include <iostream>
#include <filesystem>
#include <string>
#include <stdexcept>

#include "example.h"
// cxx generated headers
#include "cxx.h"
#include "opencv_ffi.rs.h"


void throw_error(const std::string& message) {
    std::cerr << "Error: " << message << std::endl;
    throw std::runtime_error(message);
}


void test_cpp_opencv(const std::string& image_path) {
    auto image = read_image(image_path);
    if (!image || image->empty())
    {
        throw_error("Failed to read image from " + image_path);
    }

    std::cout << "Original image loaded successfully: "
              << image->cols << "x" << image->rows
              << " with " << image->channels() << " channels." << std::endl;

    display_image(image, "Original image");
}


void test_rust_interop(const std::string& image_path) {
    auto image = read_image(image_path);
    if (!image || image->empty())
    {
        throw_error("Failed to read image from " + image_path);
    }

    // Call Rust resize function to resize to 800x600
    int new_width = 800;
    int new_height = 600;
    std::cout << "Resizing image to " << new_width << "x" << new_height << " using Rust function..." << std::endl;
    
    try {
        std::unique_ptr<CMat> resized_image = rust_resize_image(image, new_width, new_height);
        
        if (!resized_image || resized_image->empty()) {
            throw_error("Failed to create resized image");
        }
        
        std::cout << "Resized image created successfully: "
                  << resized_image->cols << "x" << resized_image->rows
                  << " with " << resized_image->channels() << " channels." << std::endl;

        // Display resized image
        display_image(resized_image, "Resized Image (800x600)");
    }
    catch (const rust::Error& e) {
        throw_error("Rust error during resize: " + std::string(e.what()));
    }
    catch (const std::exception& e) {
        throw_error("Error during resize: " + std::string(e.what()));
    }
}


int main()
{
    std::string project_root = PROJECT_ROOT; // Defined in CMakeLists.txt
    std::filesystem::path image_path = std::filesystem::path(project_root).append("assets/01.png");
    // std::filesystem::path image_path = std::filesystem::path(project_root).append("assets/02.jpg");
    std::cout << "Image path: " << image_path << std::endl;
    if (!std::filesystem::exists(image_path)) {
        throw_error("Image file does not exist: " + image_path.string());
    }

    test_cpp_opencv(image_path.string());
    test_rust_interop(image_path.string());

    return 0;
}