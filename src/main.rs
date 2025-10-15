use anyhow::{Result as AnyResult, ensure};
use librust::*;

fn main() -> AnyResult<()> {
    let img_path = "assets/01.png";
    // let img_path = "assets/02.jpg";
    ensure!(is_path_valid(img_path), "File not found: {}", img_path);

    let img = read_image(img_path)?;
    display_image(&img, "Display Image")?;

    let resized_img = resize_image(&img, 800, 600)?;
    display_image(&resized_img, "Resized Image (800x600)")?;

    Ok(())
}
