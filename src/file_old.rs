use image::ImageBuffer;
use std::{fs, path::Path};

pub fn write_to_file(filename: &str, data: Vec<f64>, width: usize, height: usize, heightmap: bool) {
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
            let value = ((i * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8;
            let pixel;
            if heightmap {
                pixel = image::Rgb([value, value, value]);
            } else {
                let mut red = 0;
                let green;
                let mut blue = 0;
                if value < 30 {
                    red = 255 - value;
                    blue = 255 - value;
                    green = 255 - value;
                } else if value < 70 {
                    blue = value;
                    green = value / 3;
                } else if value < 90 {
                    blue = value;
                    green = value;
                } else if value < 150 {
                    red = value;
                    green = value;
                } else if value < 200 {
                    red = value;
                    green = value / 2;
                } else {
                    red = value;
                    green = value / 4;
                }
                pixel = image::Rgb([red, green, blue]);
            }
            output.put_pixel(x as u32, y as u32, pixel);
        }
    }

    output.save(&file_path).unwrap_or_default();
}
