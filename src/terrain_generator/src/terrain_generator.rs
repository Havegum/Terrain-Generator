#[path = "utils.rs"]
mod utils;

use wasm_bindgen::prelude::*;
use bracket_noise::prelude::*;
use bracket_random::prelude::*;
use std::f64::consts::PI;
extern crate web_sys;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct TerrainGenerator {
    noise: FastNoise,
    noise_theta: FastNoise,
    noise_length: FastNoise,
    // noise_resources: FastNoise,
    rng: RandomNumberGenerator,
}


#[wasm_bindgen]
impl TerrainGenerator {

    #[wasm_bindgen(constructor)]
    pub fn new (seed: Option<u32>) -> TerrainGenerator {
        utils::set_panic_hook();

        let seed = match seed {
            None => 123456 as u64,
            Some(seed) => seed as u64,
        };

        let mut noise = FastNoise::seeded(seed);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(1.0);

        let mut noise_theta = FastNoise::seeded(seed + 1);
        noise_theta.set_noise_type(NoiseType::Simplex);
        noise_theta.set_frequency(2.0);

        let mut noise_length = FastNoise::seeded(seed + 2);
        noise_length.set_noise_type(NoiseType::Simplex);
        noise_length.set_frequency(2.0);

        let mut noise_resources = FastNoise::seeded(seed + 3);
        noise_resources.set_noise_type(NoiseType::Simplex);
        noise_resources.set_frequency(2.0);

        let rng = RandomNumberGenerator::seeded(seed);

        TerrainGenerator {
            noise,
            noise_theta,
            noise_length,
            // noise_resources,
            rng,
        }
    }


    pub fn fractal_noise (&self, x: f32, y: f32) -> f32 {
        let force = 0.25; // magic
        let wavyness = 5e-1; // magic

        let theta = self.noise_theta.get_noise(x * force, y * force);
        let length = self.noise_length.get_noise(x * force, y * force);

        let x = x + theta.cos() * length * wavyness;
        let y = y + theta.sin() * length * wavyness;

        self.noise.get_noise(x, y)
    }


    pub fn noise_single (&self, x: f32, y: f32) -> f32 {
        (self.fractal_noise(x, y) + 1.0) / 2.0
    }


    pub fn noise_array (&self, points: Vec<f32>, heights: Option<Vec<f32>>) -> Vec<f32> {
        let heights = match heights {
            None => vec![0.1, 0.2, 0.3].into(),
            Some(heights) => heights,
        };

        heights
            .iter()
            .enumerate()
            .map(|(i, height)| height + self.noise_single(points[i * 2], points[i * 2 + 1]))
            .collect()
    }


    fn check_poisson_sample (row: usize, col: usize, cols: usize, rows: usize, sample: &[f64; 2], grid: &Vec<Vec<[f64; 2]>> , min_offset: f64) -> bool {
        let euclidean = |a: &[f64; 2], b: &[f64; 2]| ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt();
        'i_loop: for i in ([-1, 0, 1] as [i8; 3]).iter() {
            'j_loop: for j in ([-1, 0, 1] as [i8; 3]).iter() {

                let neighbor_row = match i {
                    -1 => row.checked_sub(1),
                    1 => row.checked_add(1),
                    _ => Some(row),
                };

                let neighbor_row = match neighbor_row {
                    Some(row) => row,
                    None => continue 'i_loop,
                };

                if neighbor_row >= rows { continue 'i_loop; }

                let neighbor_col = match j {
                    -1 => col.checked_sub(1),
                    1 => col.checked_add(1),
                    _ => Some(col),
                };

                let neighbor_col = match neighbor_col {
                    Some(col) => col,
                    None => continue 'j_loop,
                };
                if neighbor_col >= cols { continue 'j_loop; }
                let neighbor_i = neighbor_row.wrapping_add(cols * neighbor_col);

                for neighbor in grid[neighbor_i].iter() {
                    let dist = euclidean(&sample, neighbor);
                    if dist < min_offset {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn sample_poisson_points (
        &mut self,
        k: usize,
        size: f64,
        width: f64,
        height: f64,
        min_offset: f64,
        point: &[f64; 2],
        grid: &mut Vec<Vec<[f64; 2]>>,
    ) -> Vec<[f64; 2]>
    {
        let mut rng = || self.rng.rand::<f64>();
        let mut new_points: Vec<[f64; 2]> = vec![];

        let cols = (width / size).floor() as usize;
        let rows = (height / size).floor() as usize;

        for _ in 0..k {
            // Get a sample at some random angle and distance from `point`
            let theta = rng() * PI * 2.0;
            let offset = size + rng() * min_offset;
            let x = point[0] + theta.cos() * offset;
            let y = point[1] + theta.sin() * offset;

            // If out of lower bounds, keep looking.
            if 0.0 > x || 0.0 > y { continue; }

            let sample = [x, y];
            let col = (x / size).floor() as usize;
            let row = (y / size).floor() as usize;

            // If out of upper bounds, keep looking.
            if row >= rows || col >= cols { continue; }

            if TerrainGenerator::check_poisson_sample(row, col, cols, rows, &sample, &grid, min_offset) == false {
                continue; // Check if too close to existing samples. If point is not valid, keep looking.
            }
            // push sample in
            grid[row + col * cols].push(sample);
            new_points.push(sample);
        }

        new_points
    }

    pub fn poisson_disc_points (&mut self, radius: f64, sea_level: f64, width: f64, height: f64) -> Vec<f64> {
        let size = radius / (2 as f64).sqrt();

        let cols = (width / size).floor();
        let rows = (height / size).floor();

        let mut grid: Vec<Vec<[f64; 2]>> = vec![vec![]; (rows * cols) as usize];
        let mut active: Vec<[f64; 2]> = Vec::new();
        let mut points: Vec<f64> = Vec::new();

        let x = self.rng.rand::<f64>() * width as f64;
        let y = self.rng.rand::<f64>() * height as f64;
        let i = (x / size).floor();
        let j = (y / size).floor();

        let pos = [x, y];
        grid[(i + j * cols) as usize].push(pos);
        active.push(pos);
        points.extend(pos.iter());

        let offset_magnitude = |h: f32| -> f64 {
            if h > sea_level as f32 {
                h as f64
            } else {
                1.0 - h as f64
            }
        };


        while active.len() > 0 {
            let rand_i = (self.rng.rand::<f64>() * active.len() as f64).floor() as usize;
            let point = &active[rand_i];
            let min_offset = size * offset_magnitude(self.noise_single(point[0] as f32, point[1] as f32));
            let new_points = self.sample_poisson_points(30, size, width, height, min_offset, &point, &mut grid);

            for sample in new_points.iter() {
                points.extend(sample.iter());
            }
            active.extend(new_points.iter());
            active.remove(rand_i);
        }

        points.extend(vec![
            width, height,
            width, 0.0,
            0.0, height,
            0.0, 0.0
        ]);
        points
    }
}
