use image::ImageBuffer;
use std::{fs, path::Path};

fn temperature_to_color(temperature: u8) -> (u8, u8, u8) {
    let x = (temperature as f32) / 255.0;

    let blue = (0, 0, 255);
    let cyan = (0, 255, 255);
    let green = (0, 255, 0);
    let yellow = (255, 255, 0);
    let red = (255, 0, 0);

    let (r, g, b) = if x < 0.2 {
        interpolate(blue, cyan, (x / 0.2) as f32)
    } else if x < 0.4 {
        interpolate(cyan, green, ((x - 0.2) / 0.2) as f32)
    } else if x < 0.6 {
        interpolate(green, yellow, ((x - 0.4) / 0.2) as f32)
    } else if x < 0.8 {
        interpolate(yellow, red, ((x - 0.6) / 0.2) as f32)
    } else {
        red
    };

    (r, g, b)
}

fn interpolate(color1: (u8, u8, u8), color2: (u8, u8, u8), factor: f32) -> (u8, u8, u8) {
    let r = ((1.0 - factor) * color1.0 as f32 + factor * color2.0 as f32) as u8;
    let g = ((1.0 - factor) * color1.1 as f32 + factor * color2.1 as f32) as u8;
    let b = ((1.0 - factor) * color1.2 as f32 + factor * color2.2 as f32) as u8;
    (r, g, b)
}

pub fn write_to_file(
    filename: &str,
    data: &Vec<f64>,
    width: usize,
    height: usize,
    heightmap: bool,
    color: [f64; 3],
) {
    let target_dir = Path::new("example_images/");

    if !target_dir.exists() {
        fs::create_dir(target_dir).expect("failed to create example_images directory");
    }

    //concatenate the directory to the filename string
    let directory: String = "example_images/".to_owned();
    let file_path = directory + filename;

    let mut output = ImageBuffer::new(width as u32, height as u32);
    for y in 1..height {
        for x in 1..width {
            let i = data[y * width + x];
            let pixel;
            if heightmap {
                let mut pixel_data = [0u8; 3];
                for j in 0..3 {
                    let val = (i * 255.0 * color[j]).clamp(0.0, 255.0) as u8;
                    pixel_data[j] = val;
                }
                pixel = image::Rgb(pixel_data);
            } else {
                let (red, green, blue) = temperature_to_color((i * 255.0).clamp(0.0, 255.0) as u8);
                pixel = image::Rgb([red, green, blue]);
            }
            output.put_pixel(x as u32, y as u32, pixel);
        }
    }

    output.save(&file_path).unwrap_or_default();
}
