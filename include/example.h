#pragma once

#include <opencv2/opencv.hpp>
#include <string>
#include <memory>

using CMat = cv::Mat;

struct MatInfo {
    int rows;
    int cols;
    int type;
    size_t step;
    const unsigned char* data;
};

std::unique_ptr<CMat> read_image(const std::string& image_path);
void display_image(const std::unique_ptr<CMat>& image, const std::string& window_name);
MatInfo get_mat_info(const std::unique_ptr<CMat>& mat);
std::unique_ptr<CMat> create_mat_from_info(const MatInfo& info);
