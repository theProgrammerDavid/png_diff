use image::{GenericImageView, ImageBuffer, Rgba};
use std::path::Path;
use types::ProgramData;
mod types;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the PNG image to use as a base
    #[arg(short, long)]
    original_image_path: String,

    /// path to the new PNG image
    #[arg(short, long)]
    new_imagepath: String,

    /// path to write the output PNG heatmap
    #[arg(short, long)]
    path_to_heatmap: String,

    /// intensity of the heatmap, 255 is fully opaque and 0 is fully transparent
    #[arg(short,long)]
    intensity: u8
}

fn main() {
    let args = Args::parse();

    let init_data = ProgramData {
        original_image_path: args.original_image_path,
        new_imagepath: args.new_imagepath,
        heatmap_path: args.path_to_heatmap,
        heatmap_intensity: args.intensity
    };

    // Paths to the images
    let img1_path = Path::new(&init_data.original_image_path);
    let img2_path = Path::new(&init_data.new_imagepath);

    // Load the images
    let img1 = image::open(&img1_path).expect(&format!(
        "Failed to open {}",
        &init_data.original_image_path
    ));
    let img2 =
        image::open(&img2_path).expect(&format!("Failed to open {}", &init_data.new_imagepath));

    if img1.dimensions() != img2.dimensions() {
        panic!("Images have different dimensions");
    }

    let (width, height) = img1.dimensions();

    // Create an image to store the differences (heatmap)
    let mut heatmap_img = ImageBuffer::from_fn(width, height, |_x, _y| Rgba([255, 255, 255, 255]));
    let mut difference_found = false;
    let mut pixels_changed: u64 = 0;

    for x in 0..width {
        for y in 0..height {
            let px1 = img1.get_pixel(x, y);
            let px2 = img2.get_pixel(x, y);

            let diff = calculate_difference(px1, px2);

            if diff > 0 {
                pixels_changed += 1;

                if !difference_found {
                    difference_found = diff > 0;
                }
                let heatmap_color = blend_with_white(Rgba([255, 0, 0, 255]), diff);
                heatmap_img.put_pixel(x, y, heatmap_color);
            } else {
                heatmap_img.put_pixel(x, y, Rgba([px1[0], px1[1], px1[2], init_data.heatmap_intensity]));
            }
        }
    }

    // Create the final image with three parts: img1, heatmap, img2
    let mut final_img = ImageBuffer::new(width * 3, height);

    // Copy img1 to the final image
    for x in 0..width {
        for y in 0..height {
            final_img.put_pixel(x, y, img1.get_pixel(x, y));
        }
    }

    // Copy heatmap to the final image
    for x in 0..width {
        for y in 0..height {
            final_img.put_pixel(x + width, y, *heatmap_img.get_pixel(x, y));
        }
    }

    // Copy img2 to the final image
    for x in 0..width {
        for y in 0..height {
            final_img.put_pixel(x + 2 * width, y, img2.get_pixel(x, y));
        }
    }

    // Save the final image
    final_img
        .save(init_data.heatmap_path)
        .expect("Failed to save diff.png");

        println!(
            r#"{{"non_zero_diff_found": {}, "pixels_changed": {}}}"#,
            difference_found, pixels_changed
        );
}

fn calculate_difference(px1: Rgba<u8>, px2: Rgba<u8>) -> u8 {
    let r_diff = (px1[0] as i16 - px2[0] as i16).abs() as u8;
    let g_diff = (px1[1] as i16 - px2[1] as i16).abs() as u8;
    let b_diff = (px1[2] as i16 - px2[2] as i16).abs() as u8;

    let intensity = ((r_diff as u16 + g_diff as u16 + b_diff as u16) / 3) as u8;

    intensity
}

fn blend_with_white(color: Rgba<u8>, intensity: u8) -> Rgba<u8> {
    // Blend the color with white based on the intensity
    let r = ((255 - color[0] as u16) * (255 - intensity) as u16 / 255 + color[0] as u16) as u8;
    let g = ((255 - color[1] as u16) * (255 - intensity) as u16 / 255 + color[1] as u16) as u8;
    let b = ((255 - color[2] as u16) * (255 - intensity) as u16 / 255 + color[2] as u16) as u8;

    Rgba([r, g, b, 255])
}