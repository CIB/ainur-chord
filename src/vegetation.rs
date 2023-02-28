const SEA_LEVEL: f64 = -0.5;

pub fn modify_rainfall_by_heightmap(rainfall: &mut [f64], heightmap: &[f64], temperature: &[f64]) {
    let scale_factor = 2.0;

    for (i, height) in heightmap.iter().enumerate() {
        let modified_height = height - 1.0;
        let modified_rainfall = (1.0 - modified_height.powi(2)) * scale_factor;
        rainfall[i] *= modified_rainfall.max(0.0);

        if temperature[i] > 0.6 {
            let temperature_factor = 1.0 - (temperature[i] - 0.6) / 0.4;
            rainfall[i] *= temperature_factor;
        }
    }
}

pub fn compute_vegetation(
    heightmap: &[f64],
    rainfall: &[f64],
    temperature: &[f64],
    map_width: usize,
    map_height: usize,
) -> Vec<f64> {
    let mut vegetation = vec![0.0; map_width * map_height];

    for y in 0..map_height {
        for x in 0..map_width {
            let i = y * map_width + x;
            let height = heightmap[i];
            let rain = rainfall[i];
            let temp = temperature[i];

            // Compute vegetation based on height, rainfall, and temperature
            let mut veg = rain;
            if temp < 0.4 {
                veg *= temp / 0.4;
            }
            if height > 0.8 {
                veg *= 1.0 - (height - 0.8) / 0.2;
            }
            if height < 0.1 {
                veg *= height / 0.1;
            }

            vegetation[i] = veg * 0.6;
        }
    }

    vegetation
}
