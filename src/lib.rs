pub mod opencv_ffi;

use opencv_ffi::cv_conversion;
use opencv_ffi::ffi;

use anyhow::Result as AnyResult;
use opencv::prelude::*;


// ------------ Pure Rust Functions ------------

pub fn is_path_valid(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn resize_image(
    img: &opencv::core::Mat,
    width: i32,
    height: i32,
) -> AnyResult<opencv::core::Mat> {
    let mut resized_img = opencv::core::Mat::default();
    let size = opencv::core::Size::new(width, height);
    opencv::imgproc::resize(
        img,
        &mut resized_img,
        size,
        0.0,
        0.0,
        opencv::imgproc::INTER_LINEAR,
    )?;
    Ok(resized_img)
}

// ------------ Call C++ function ---------

pub fn flip_image_with_cpp(img: &opencv::core::Mat) -> AnyResult<opencv::core::Mat> {
    if img.empty() {
        anyhow::bail!("Cannot process empty Mat");
    }

    unsafe {
        // Rust Mat -> C++ Mat (Zero-copy conversion)
        let cpp_ref = cv_conversion::zero_copy_rust_to_cpp_ref(img)?;

        // Call C++ function
        let flipped_cpp_mat = ffi::flip_image_cpp(cpp_ref);

        // C++ Mat -> Rust Mat
        let rust_mat = cv_conversion::safe_convert_cpp_to_rust(&flipped_cpp_mat)?;

        Ok(rust_mat)
    }
}

// -------------- Unit Tests ----------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_path_valid() {
        assert!(is_path_valid("."));
        assert!(!is_path_valid("non_existent_file_123456.txt"));
    }

    #[test]
    fn test_resize_image() {
        use opencv::imgcodecs::*;

        let img_path = "assets/01.png";
        assert!(
            is_path_valid(img_path),
            "Test image not found: {}",
            img_path
        );

        let img = imread(img_path, IMREAD_COLOR).unwrap();

        let resized_img = resize_image(&img, 100, 100).unwrap();
        assert_eq!(resized_img.rows(), 100);
        assert_eq!(resized_img.cols(), 100);
    }

    #[test]
    fn test_flip_image_with_cpp() {
        use opencv::imgcodecs::*;

        let img_path = "assets/01.png";
        assert!(
            is_path_valid(img_path),
            "Test image not found: {}",
            img_path
        );

        let img = imread(img_path, IMREAD_COLOR).unwrap();
        let flipped_img = flip_image_with_cpp(&img).unwrap();

        assert_eq!(img.rows(), flipped_img.rows());
        assert_eq!(img.cols(), flipped_img.cols());
    }
}
