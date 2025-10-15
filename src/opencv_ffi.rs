use crate::is_path_valid;
use anyhow::{Result as AnyResult, anyhow};
use opencv::prelude::*;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("example.h");

        type CMat;
        type MatInfo = super::MatInfo;

        fn read_image(image_path: &CxxString) -> UniquePtr<CMat>;
        fn get_mat_info(mat: &UniquePtr<CMat>) -> MatInfo;
        fn create_mat_from_info(info: &MatInfo) -> UniquePtr<CMat>;
    }

    extern "Rust" {
        fn rust_resize_image(input_mat: &UniquePtr<CMat>, width: i32, height: i32) -> Result<UniquePtr<CMat>>;
    }
}

// ------ Type Definitions ------

#[repr(C)]
#[derive(Debug, Clone)]
struct MatInfo {
    rows: i32,
    cols: i32,
    type_: i32,
    step: usize,
    data: *const u8,
}

unsafe impl cxx::ExternType for MatInfo {
    type Id = cxx::type_id!("MatInfo");
    type Kind = cxx::kind::Trivial;
}

// ------ From/Into Trait Implementations ------

impl TryFrom<&opencv::core::Mat> for MatInfo {
    type Error = anyhow::Error;

    fn try_from(mat: &opencv::core::Mat) -> AnyResult<Self> {
        Ok(MatInfo {
            rows: mat.rows(),
            cols: mat.cols(),
            type_: mat.typ(),
            step: mat
                .step1(0)
                .map_err(|e| anyhow!("Failed to get step: {}", e))?,
            data: mat
                .ptr(0)
                .map_err(|e| anyhow!("Failed to get data pointer: {}", e))?,
        })
    }
}

impl TryFrom<MatInfo> for opencv::core::Mat {
    type Error = anyhow::Error;

    fn try_from(mat_info: MatInfo) -> AnyResult<Self> {
        (&mat_info).try_into()
    }
}

impl TryFrom<&MatInfo> for opencv::core::Mat {
    type Error = anyhow::Error;

    fn try_from(mat_info: &MatInfo) -> AnyResult<Self> {
        if mat_info.data.is_null() || mat_info.rows <= 0 || mat_info.cols <= 0 {
            return Err(anyhow!(
                "Invalid MatInfo data: null pointer or invalid dimensions"
            ));
        }

        unsafe {
            // Calculate total data size
            let total_size = mat_info.step * (mat_info.rows as usize);

            // Create data copy to avoid lifetime issues
            let data_slice = std::slice::from_raw_parts(mat_info.data, total_size);
            let data_copy = data_slice.to_vec();

            // Create new Mat and copy data
            let mut mat =
                opencv::core::Mat::new_rows_cols(mat_info.rows, mat_info.cols, mat_info.type_)
                    .map_err(|e| anyhow!("Failed to create new Mat: {}", e))?;

            // Get mutable reference to mat data and copy
            let elem_size = mat
                .elem_size()
                .map_err(|e| anyhow!("Failed to get element size: {}", e))?
                as usize;
            let mat_size = mat.total() * elem_size;
            let copy_size = std::cmp::min(total_size, mat_size);

            if copy_size > 0 {
                let mat_ptr = mat
                    .ptr_mut(0)
                    .map_err(|e| anyhow!("Failed to get mutable pointer: {}", e))?;
                std::ptr::copy_nonoverlapping(data_copy.as_ptr(), mat_ptr, copy_size);
            }

            Ok(mat)
        }
    }
}

impl TryFrom<&MatInfo> for cxx::UniquePtr<ffi::CMat> {
    type Error = anyhow::Error;

    fn try_from(mat_info: &MatInfo) -> AnyResult<Self> {
        let cpp_mat = ffi::create_mat_from_info(mat_info);

        if cpp_mat.is_null() {
            return Err(anyhow!("Failed to create C++ Mat from MatInfo"));
        }

        Ok(cpp_mat)
    }
}

impl TryFrom<MatInfo> for cxx::UniquePtr<ffi::CMat> {
    type Error = anyhow::Error;

    fn try_from(mat_info: MatInfo) -> AnyResult<Self> {
        (&mat_info).try_into()
    }
}

impl From<&cxx::UniquePtr<ffi::CMat>> for MatInfo {
    fn from(cpp_mat: &cxx::UniquePtr<ffi::CMat>) -> Self {
        ffi::get_mat_info(cpp_mat)
    }
}

// ------ Private Functions ------

#[allow(dead_code)]
fn rust_image_to_cpp_image(rust_mat: &opencv::core::Mat) -> AnyResult<cxx::UniquePtr<ffi::CMat>> {
    MatInfo::try_from(rust_mat)?.try_into()
}

fn cpp_image_to_rust_image(input_mat: &cxx::UniquePtr<ffi::CMat>) -> AnyResult<opencv::core::Mat> {
    MatInfo::from(input_mat).try_into()
}

// ------ Public Functions ------

pub fn read_image(path: &str) -> AnyResult<opencv::core::Mat> {
    if !is_path_valid(path) {
        return Err(anyhow!("Image path does not exist: {}", path));
    }

    cxx::let_cxx_string!(c_path = path);
    let cpp_mat = ffi::read_image(&c_path);

    if cpp_mat.is_null() {
        return Err(anyhow!("Failed to read image from path: {}", path));
    }

    cpp_image_to_rust_image(&cpp_mat)
}

// ------ C++ Callable Functions ------

fn rust_resize_image(cimg: &cxx::UniquePtr<ffi::CMat>, width: i32, height: i32) -> AnyResult<cxx::UniquePtr<ffi::CMat>> {
    // Convert C++ Mat to Rust Mat
    let rimg = cpp_image_to_rust_image(cimg)?;

    // Perform resize operation
    let resized = crate::resize_image(&rimg, width, height)
        .map_err(|e| anyhow!("Resize operation failed: {}", e))?;

    // Convert back to C++ Mat via MatInfo
    let result_info: MatInfo = (&resized).try_into()?;
    let cresult = result_info.try_into()?;

    Ok(cresult)
}

// ------ Unit Tests ------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_path_valid;

    #[test]
    fn test_cpp_image_to_rust_image() {
        let img_path = "assets/01.png";

        // Check if test image exists
        if !is_path_valid(img_path) {
            println!("Skipping test: image {} not found", img_path);
            return;
        }

        // Use C++ function to read image
        cxx::let_cxx_string!(img_cpath = img_path);
        let cpp_img = ffi::read_image(&img_cpath);

        // Test conversion function
        match cpp_image_to_rust_image(&cpp_img) {
            Ok(rust_mat) => {
                // Verify converted image properties
                assert!(rust_mat.rows() > 0, "Image rows should be > 0");
                assert!(rust_mat.cols() > 0, "Image cols should be > 0");
                assert!(!rust_mat.empty(), "Converted Mat should not be empty");

                // Verify image type is reasonable (usually CV_8UC1, CV_8UC3, CV_8UC4, etc.)
                let mat_type = rust_mat.typ();
                assert!(mat_type >= 0, "Mat type should be non-negative");

                println!(
                    "Conversion test successful: {}x{}, type: {}",
                    rust_mat.rows(),
                    rust_mat.cols(),
                    mat_type
                );
            }
            Err(e) => {
                panic!("Image conversion failed: {}", e);
            }
        }
    }

    #[test]
    fn test_read_image() {
        let valid_path = "assets/01.png";
        let invalid_path = "non_existent_file_123456.png";

        // Test valid path
        match read_image(valid_path) {
            Ok(mat) => {
                assert!(mat.rows() > 0, "Image rows should be > 0");
                assert!(mat.cols() > 0, "Image cols should be > 0");
                assert!(!mat.empty(), "Read Mat should not be empty");
                println!(
                    "Read test successful: {}x{}, type: {}",
                    mat.rows(),
                    mat.cols(),
                    mat.typ()
                );
            }
            Err(e) => {
                panic!("Failed to read valid image: {}", e);
            }
        }

        // Test invalid path
        match read_image(invalid_path) {
            Ok(_) => {
                panic!("Reading invalid path should fail but succeeded");
            }
            Err(e) => {
                println!("Correctly caught invalid path error: {}", e);
            }
        }
    }

    #[test]
    fn test_rust_image_to_cpp_image() -> AnyResult<()> {
        let img_path = "assets/01.png";

        // Check if test image exists
        if !is_path_valid(img_path) {
            println!("Skipping test: image {} not found", img_path);
            return Ok(());
        }

        // Read image and save original properties
        let rust_mat = read_image(img_path)?;
        let (original_rows, original_cols, original_type) =
            (rust_mat.rows(), rust_mat.cols(), rust_mat.typ());

        // Convert to C++ Mat
        let cpp_mat = rust_image_to_cpp_image(&rust_mat)?;
        assert!(!cpp_mat.is_null(), "Converted C++ Mat should not be null");

        // Convert back to Rust Mat for verification
        let converted_back = cpp_image_to_rust_image(&cpp_mat)?;

        // Verify properties remain consistent before and after conversion
        assert_eq!(converted_back.rows(), original_rows, "Rows should remain consistent");
        assert_eq!(converted_back.cols(), original_cols, "Cols should remain consistent");
        assert_eq!(converted_back.typ(), original_type, "Type should remain consistent");

        println!(
            "Rust -> C++ -> Rust conversion test successful: {}x{}, type: {}",
            converted_back.rows(),
            converted_back.cols(),
            converted_back.typ()
        );

        Ok(())
    }
}
