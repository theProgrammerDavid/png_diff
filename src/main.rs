use field_count::FieldCount;
use image::{GenericImageView, ImageBuffer, Rgba};
use std::env;
use std::path::Path;
use std::process;
use types::ProgramData;

mod types;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != (ProgramData::field_count() + 1) {
        eprintln!("Invalid number of arguments");
        eprintln!("Correct usage is");
        eprintln!("png_diff <path_to_original> <path_to_new> <path_to_output_heatmap>");
        process::exit(1);
    }

    let init_data = ProgramData {
        original_image_path: args[1].clone(),
        new_imagepath: args[2].clone(),
        heatmap_path: args[3].clone(),
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

    // Ensure the images have the same dimensions
    if img1.dimensions() != img2.dimensions() {
        panic!("Images have different dimensions");
    }

    let (width, height) = img1.dimensions();

    // Create an image to store the differences (heatmap)
    let mut heatmap_img = ImageBuffer::new(width, height);
    let mut difference_found = false;
    // let mut count_zero:i32 = 0;
    // let mut count_non_zero:i32 = 0;

    // Calculate the differences and generate the heatmap
    for x in 0..width {
        for y in 0..height {
            let px1 = img1.get_pixel(x, y);
            let px2 = img2.get_pixel(x, y);

            let diff = calculate_difference(px1, px2);

            if !difference_found {
                difference_found = diff > 0;

                // if diff > 0 {
                //     count_non_zero += 1;
                // }
                // else {
                //     count_zero += 1;
                // }
            }

            // Create a heatmap color based on the difference
            let heatmap_color = Rgba([255, 0, 0, diff]);

            heatmap_img.put_pixel(x, y, heatmap_color);
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

    println!("non zero diff found: {}", difference_found);
}

fn calculate_difference(px1: Rgba<u8>, px2: Rgba<u8>) -> u8 {
    let r_diff = (px1[0] as i16 - px2[0] as i16).abs() as u8;
    let g_diff = (px1[1] as i16 - px2[1] as i16).abs() as u8;
    let b_diff = (px1[2] as i16 - px2[2] as i16).abs() as u8;

    // Calculate the intensity of the difference
    let intensity = ((r_diff as u16 + g_diff as u16 + b_diff as u16) / 3) as u8;

    intensity
}
