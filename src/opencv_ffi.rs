use crate::resize_image;

use anyhow::Result as AnyResult;
use cxx::UniquePtr;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("opencv_ffi.h");

        type CMat; // alias for cv::Mat

        // ------------- Conversion Functions -------------

        fn rust_mat_to_cpp_ref(rust_mat_ptr_addr: usize) -> *const CMat;
        fn rust_to_cpp_safe(rust_mat_ptr_addr: usize) -> UniquePtr<CMat>;
        fn cpp_to_rust_safe(cpp_mat: &CMat) -> usize;

        // ------------- C++ OpenCV Functions -------------

        fn flip_image_cpp(input_mat: &CMat) -> UniquePtr<CMat>;
    }

    extern "Rust" {
        fn resize_image_rust(input_mat: &CMat, width: i32, height: i32) -> Result<UniquePtr<CMat>>;
    }
}

/// Conversion between Rust OpenCV Mat and C++ OpenCV Mat
pub mod conversion {
    use super::*;
    use opencv::prelude::*;

    pub use super::ffi::CMat;

    // ----------- Core Conversion Functions -----------

    /// Rust Mat -> C++ Mat (safe copy)
    pub fn safe_convert_rust_to_cpp(
        rust_mat: &opencv::core::Mat,
    ) -> AnyResult<cxx::UniquePtr<CMat>> {
        if rust_mat.empty() {
            anyhow::bail!("Cannot convert empty Mat");
        }

        let ptr_addr = rust_mat.as_raw_Mat() as usize;
        Ok(super::ffi::rust_to_cpp_safe(ptr_addr))
    }

    /// C++ Mat -> Rust Mat (safe copy)
    pub fn safe_convert_cpp_to_rust(cpp_mat: &CMat) -> AnyResult<opencv::core::Mat> {
        let ptr_addr = super::ffi::cpp_to_rust_safe(cpp_mat);
        if ptr_addr == 0 {
            anyhow::bail!("Failed to convert C++ Mat: null pointer returned");
        }

        unsafe { Ok(from_ptr_addr(ptr_addr)) }
    }

    /// Rust Mat -> C++ Mat reference (zero-copy)
    ///
    /// ⚠️ Must be called within unsafe block, ensure Rust Mat lifetime covers entire usage period
    pub unsafe fn zero_copy_rust_to_cpp_ref(rust_mat: &opencv::core::Mat) -> AnyResult<&CMat> {
        if rust_mat.empty() {
            anyhow::bail!("Cannot convert empty Mat");
        }

        let ptr_addr = rust_mat.as_raw_Mat() as usize;
        let cpp_ref_ptr = super::ffi::rust_mat_to_cpp_ref(ptr_addr);
        Ok(unsafe { &*cpp_ref_ptr })
    }

    // ---------- Internal Helper Functions ------------

    /// Create Rust Mat from C++ pointer address
    unsafe fn from_ptr_addr(ptr_addr: usize) -> opencv::core::Mat {
        if ptr_addr == 0 {
            return opencv::core::Mat::default();
        }

        let cv_mat_ptr = ptr_addr as *mut std::ffi::c_void;
        unsafe { opencv::core::Mat::from_raw(cv_mat_ptr) }
    }
}

// ---------------- Functions exposed to C++  -----------------

fn resize_image_rust(
    input_mat: &ffi::CMat,
    width: i32,
    height: i32,
) -> AnyResult<UniquePtr<ffi::CMat>> {
    let rust_mat = conversion::safe_convert_cpp_to_rust(input_mat)?;
    let resized_mat = resize_image(&rust_mat, width, height)?;

    // Convert the resized Rust Mat back to C++ Mat (safe copy)
    conversion::safe_convert_rust_to_cpp(&resized_mat)
}

// -------------- Unit Tests ----------------

#[cfg(test)]
mod tests {
    use super::*;
    use opencv::imgcodecs::*;
    use opencv::prelude::*;

    #[test]
    fn test_conversions() -> AnyResult<()> {
        use super::conversion::*;

        let img_path = "assets/01.png";
        if !std::path::Path::new(img_path).exists() {
            return Ok(());
        }

        let mat = imread(img_path, IMREAD_COLOR)?;

        // Test safe conversion
        let cpp_mat = safe_convert_rust_to_cpp(&mat)?;
        let converted = safe_convert_cpp_to_rust(&cpp_mat)?;
        assert_eq!(mat.rows(), converted.rows());
        assert_eq!(mat.cols(), converted.cols());

        // Test zero-copy conversion
        unsafe {
            let _cpp_ref = zero_copy_rust_to_cpp_ref(&mat)?;
        }

        Ok(())
    }

    #[test]
    fn test_resize_image_rust() -> AnyResult<()> {
        use super::conversion::*;
        use super::resize_image_rust;

        let img_path = "assets/01.png";
        if !std::path::Path::new(img_path).exists() {
            return Ok(());
        }

        let mat = imread(img_path, IMREAD_COLOR)?;
        let original_rows = mat.rows();
        let original_cols = mat.cols();

        // Convert to C++ Mat
        let cpp_mat = safe_convert_rust_to_cpp(&mat)?;

        // Test resize function
        let new_width = 200;
        let new_height = 150;
        let resized_cpp_mat = resize_image_rust(&cpp_mat, new_width, new_height)?;

        // Convert back to Rust Mat to verify
        let resized_rust_mat = safe_convert_cpp_to_rust(&resized_cpp_mat)?;

        assert_eq!(resized_rust_mat.rows(), new_height);
        assert_eq!(resized_rust_mat.cols(), new_width);
        assert_ne!(resized_rust_mat.rows(), original_rows);
        assert_ne!(resized_rust_mat.cols(), original_cols);

        println!("Original size: {}x{}", original_cols, original_rows);
        println!("Resized size: {}x{}", new_width, new_height);

        Ok(())
    }
}
