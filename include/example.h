#pragma once

#include <string>
#include <iostream>
#include <filesystem>
#include <stdexcept>

#include <opencv2/opencv.hpp>

// ----------- inline functions -----------

inline void throw_error(const std::string &msg)
{
    std::cerr << "Error: " << msg << std::endl;
    throw std::runtime_error(msg);
}

inline void assert_file_exists(const std::filesystem::path &path)
{
    if (!std::filesystem::exists(path))
    {
        throw_error("File does not exist: " + path.string());
    }
}

// ----------- normal functions -----------

cv::Mat flip_image(const cv::Mat &input_image);
