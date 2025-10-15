mod opencv_ffi;
pub use opencv_ffi::read_image;

pub fn is_path_valid(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn display_image(img: &opencv::core::Mat, window_name: &str) -> opencv::Result<()> {
    use opencv::highgui::*;

    imshow(window_name, img)?;
    wait_key(0)?;
    destroy_all_windows()?;
    Ok(())
}

pub fn resize_image(
    img: &opencv::core::Mat,
    width: i32,
    height: i32,
) -> opencv::Result<opencv::core::Mat> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use opencv::prelude::*;

    #[test]
    fn test_is_path_valid() {
        assert!(is_path_valid("."));
        assert!(!is_path_valid("non_existent_file_123456.txt"));
    }

    #[test]
    fn test_resize_image() -> opencv::Result<()> {
        /// Helper function to verify resize results
        fn verify_resized_image(
            resized_img: &opencv::core::Mat,
            original_img: &opencv::core::Mat,
            expected_width: i32,
            expected_height: i32,
            test_name: &str,
        ) {
            assert_eq!(resized_img.cols(), expected_width, "{} width should be correct", test_name);
            assert_eq!(resized_img.rows(), expected_height, "{} height should be correct", test_name);
            assert!(!resized_img.empty(), "{} Mat should not be empty", test_name);
            assert_eq!(resized_img.typ(), original_img.typ(), "{} image type should match original", test_name);
            
            println!("{} test passed: {}x{} -> {}x{}", test_name, 
                     original_img.rows(), original_img.cols(),
                     resized_img.rows(), resized_img.cols());
        }

        let img_path = "assets/01.png";

        // Check if test image exists
        if !is_path_valid(img_path) {
            println!("Skipping test: test image {} not found", img_path);
            return Ok(());
        }

        // Read test image
        let original_img = read_image(img_path).expect("Failed to read test image");
        let (original_rows, original_cols) = (original_img.rows(), original_img.cols());
        
        println!("Original image size: {}x{}", original_rows, original_cols);

        // Test cases: [(width, height, test_name)]
        let test_cases = [
            (800, 600, "Resize"),
            (original_cols * 2, original_rows * 2, "Upscale"),
            (original_cols / 2, original_rows / 2, "Downscale"),
        ];

        for (target_width, target_height, test_name) in test_cases {
            let resized_img = resize_image(&original_img, target_width, target_height)?;
            verify_resized_image(&resized_img, &original_img, target_width, target_height, test_name);
        }

        Ok(())
    }
}
