#include "opencv_ffi.h"

// ----------- Conversion Functions -----------

/// Convert Rust Mat pointer to C++ Mat reference (zero-copy)
const CMat *rust_mat_to_cpp_ref(uintptr_t rust_mat_ptr_addr)
{
    if (rust_mat_ptr_addr == 0)
    {
        throw_error("rust_mat_to_cpp_ref: null pointer address provided");
    }

    const cv::Mat *cv_mat = reinterpret_cast<const cv::Mat *>(rust_mat_ptr_addr);

    if (cv_mat->empty())
    {
        throw_error("rust_mat_to_cpp_ref: provided Mat is empty");
    }

    return cv_mat;
}

/// Convert Rust Mat pointer to C++ Mat copy (safe)
std::unique_ptr<CMat> rust_to_cpp_safe(uintptr_t rust_mat_ptr_addr)
{
    if (rust_mat_ptr_addr == 0)
    {
        throw_error("rust_to_cpp_safe: null pointer address provided");
    }

    const cv::Mat *source_mat = reinterpret_cast<const cv::Mat *>(rust_mat_ptr_addr);

    if (source_mat->empty())
    {
        throw_error("rust_to_cpp_safe: source Mat is empty");
    }

    auto cpp_mat = std::make_unique<cv::Mat>(*source_mat);

    return cpp_mat;
}

/// Convert C++ Mat to Rust pointer address (creates copy)
uintptr_t cpp_to_rust_safe(const CMat &cpp_mat)
{
    if (cpp_mat.empty())
    {
        throw_error("cpp_to_rust_safe: source Mat is empty");
    }

    cv::Mat *new_mat = new cv::Mat(cpp_mat);

    return reinterpret_cast<uintptr_t>(new_mat);
}

// ------------ Functions exposed to Rust ------------

std::unique_ptr<CMat> flip_image_cpp(const CMat &input_image)
{
    cv::Mat flipped_image = flip_image(input_image);
    if (flipped_image.empty())
    {
        throw_error("Failed to flip image in C++ FFI function.");
    }
    return std::make_unique<CMat>(flipped_image);
}
