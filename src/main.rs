use noise::{Fbm, NoiseFn, Perlin, Seedable};
mod file;

fn perlin_map(map_width: usize, map_height: usize, seed: u32) -> Vec<f64> {
    let mut heightmap = vec![0 as f64; map_width * map_height];
    let fbm = Fbm::<Perlin>::default().set_seed(seed);
    for y in 1..map_height {
        for x in 1..map_width {
            heightmap[y * map_width + x] = fbm.get([x as f64 / 200.0, y as f64 / 200.0]);
        }
    }
    return heightmap;
}

fn main() {
    let map_width = 2048;
    let map_height = 2048;

    let mut heightmap = perlin_map(map_width, map_height, 0);
    // let mut rainfall = perlin_map(map_width, map_height);
    let mut temperature = perlin_map(map_width, map_height, 1);

    for y in 1..map_height {
        for x in 1..map_width {
            let center_distance = ((x.abs_diff(map_width / 2) as f64).powf(2.0)
                + (y.abs_diff(map_height / 2) as f64).powf(2.0))
            .sqrt();
            heightmap[y * map_width + x] -= center_distance / 300.0 - 1.0;
        }
    }

    for y in 1..map_height {
        for x in 1..map_width {
            let value = temperature[y * map_width + x];
            let height = heightmap[y * map_width + x].max(0.0);
            temperature[y * map_width + x] = 0.6 - (y as f64 / 1500.0) - 0.5 * height + 0.2 * value;
        }
    }

    file::write_to_file("heightmap.png", heightmap, map_width, map_height, true);
    file::write_to_file("temperature.png", temperature, map_width, map_height, false);
}
