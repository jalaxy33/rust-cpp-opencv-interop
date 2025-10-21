#include "example.h"


cv::Mat flip_image(const cv::Mat &input_image)
{
    cv::Mat flipped_image;
    cv::flip(input_image, flipped_image, 1); // Flip around y-axis
    return flipped_image;
}


