#pragma once
#include <memory>
#include <opencv2/opencv.hpp>

#include "example.h"

// ----------- Opaque Type -----------

using CMat = cv::Mat;

// ----------- Conversion Functions -----------

const CMat *rust_mat_to_cpp_ref(uintptr_t rust_mat_ptr_addr);
std::unique_ptr<CMat> rust_to_cpp_safe(uintptr_t rust_mat_ptr_addr);
uintptr_t cpp_to_rust_safe(const CMat &cpp_mat);

// ----------- Functions exposed to Rust -----------

std::unique_ptr<CMat> flip_image_cpp(const CMat &input_image);
