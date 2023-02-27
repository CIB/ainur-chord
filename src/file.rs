use image::ImageBuffer;
use std::{fs, path::Path};

fn temperature_to_hue(temperature: u8) -> u8 {
    let x = 1.0 - (temperature as f32) / 255.0;

    let blue_hue = 170;
    let teal_hue = 127;
    let green_hue = 85;
    let yellow_hue = 43;
    let orange_hue = 21;
    let red_hue = 0;

    let hue = if x < 0.2 {
        blue_hue + ((teal_hue - blue_hue) as f32 * x * 5.0) as u8
    } else if x < 0.4 {
        teal_hue + ((green_hue - teal_hue) as f32 * (x - 0.2) * 5.0) as u8
    } else if x < 0.6 {
        green_hue + ((yellow_hue - green_hue) as f32 * (x - 0.4) * 5.0) as u8
    } else if x < 0.8 {
        yellow_hue + ((orange_hue - yellow_hue) as f32 * (x - 0.6) * 5.0) as u8
    } else if x < 1.0 {
        orange_hue + ((red_hue - orange_hue) as f32 * (x - 0.8) * 5.0) as u8
    } else {
        red_hue
    };

    hue
}

fn hsb_to_rgb(hue: u8, saturation: u8, brightness: u8) -> (u8, u8, u8) {
    let hue = hue as f32 / 255.0;
    let saturation = saturation as f32 / 255.0;
    let brightness = brightness as f32 / 255.0;

    let c = brightness * saturation;
    let x = c * (1.0 - ((hue * 6.0) % 2.0 - 1.0).abs());
    let m = brightness - c;

    let (r, g, b) = if hue < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if hue < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if hue < 3.0 / 6.0 {
        (0.0, c, x)
    } else if hue < 4.0 / 6.0 {
        (0.0, x, c)
    } else if hue < 5.0 / 6.0 {
        (x, 0.0, c)
    } else if hue <= 1.0 {
        (c, 0.0, x)
    } else {
        (0.0, 0.0, 0.0)
    };

    let (r, g, b) = (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    );

    (r, g, b)
}

fn temperature_to_color(temperature: u8) -> (u8, u8, u8) {
    let hue = temperature_to_hue(temperature);
    let saturation = 255;
    let brightness = 255;

    hsb_to_rgb(hue, saturation, brightness)
}

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
                let (red, green, blue) = temperature_to_color(value);
                pixel = image::Rgb([red, green, blue]);
            }
            output.put_pixel(x as u32, y as u32, pixel);
        }
    }

    output.save(&file_path).unwrap_or_default();
}
