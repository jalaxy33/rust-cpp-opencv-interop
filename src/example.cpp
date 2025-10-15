#include "example.h"

std::unique_ptr<CMat> read_image(const std::string &image_path)
{
    auto image = std::make_unique<CMat>(cv::imread(image_path, cv::IMREAD_COLOR));
    if (image->empty())
    {
        throw std::runtime_error("Could not open or find the image: " + image_path);
    }
    return image;
}

void display_image(const std::unique_ptr<CMat> &image, const std::string &window_name)
{
    cv::imshow(window_name, *image);
    cv::waitKey(0);
}

MatInfo get_mat_info(const std::unique_ptr<CMat>& mat)
{
    MatInfo info;
    if (mat && !mat->empty()) {
        info.rows = mat->rows;
        info.cols = mat->cols;
        info.type = mat->type();
        info.step = mat->step[0];
        info.data = mat->data;
    } else {
        info.rows = 0;
        info.cols = 0;
        info.type = 0;
        info.step = 0;
        info.data = nullptr;
    }
    return info;
}

std::unique_ptr<CMat> create_mat_from_info(const MatInfo& info)
{
    if (info.data == nullptr || info.rows <= 0 || info.cols <= 0) {
        return std::make_unique<CMat>();
    }
    
    // Create new Mat and copy data
    auto mat = std::make_unique<CMat>(info.rows, info.cols, info.type);
    
    // Calculate data size to copy
    size_t data_size = info.step * info.rows;
    size_t mat_size = mat->step[0] * mat->rows;
    size_t copy_size = std::min(data_size, mat_size);
    
    // Copy data
    if (copy_size > 0) {
        std::memcpy(mat->data, info.data, copy_size);
    }
    
    return mat;
}
