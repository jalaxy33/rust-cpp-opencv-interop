use anyhow::{Result as AnyResult, ensure};
use opencv::prelude::*;

use librust::*;

fn try_pure_rust(img_path: &str) -> AnyResult<()> {
    use opencv::highgui::*;
    use opencv::imgcodecs::*;

    println!("Calling try_pure_rust...");
    ensure!(is_path_valid(img_path), "File not found: {}", img_path);

    let img = imread(img_path, IMREAD_COLOR)?;
    println!("Image size: {} x {}", img.rows(), img.cols());

    let resized_img = resize_image(&img, 800, 600)?;
    println!(
        "Resized image size: {} x {}",
        resized_img.rows(),
        resized_img.cols()
    );

    imshow("Pure Rust", &img)?;
    imshow("Pure Rust Resized (800x600)", &resized_img)?;
    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

fn try_mix_rust_cpp(img_path: &str) -> AnyResult<()> {
    use opencv::highgui::*;
    use opencv::imgcodecs::*;

    println!("Calling try_mix_rust_cpp...");
    ensure!(is_path_valid(img_path), "File not found: {}", img_path);

    let img = imread(img_path, IMREAD_COLOR)?;
    println!("Image size: {} x {}", img.rows(), img.cols());

    let flipped_img = flip_image_with_cpp(&img)?;
    println!(
        "Flipped image size: {} x {}",
        flipped_img.rows(),
        flipped_img.cols()
    );

    imshow("Mixed Rust/C++", &img)?;
    imshow("Mixed Rust/C++ Flipped", &flipped_img)?;
    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

fn main() -> AnyResult<()> {
    let img_path1 = "assets/01.png";
    let img_path2 = "assets/02.jpg";

    try_pure_rust(img_path1)?;
    try_mix_rust_cpp(img_path2)?;

    Ok(())
}
