use wasm_bindgen::prelude::*;

use super::utils;
use super::poisson;
use super::noise::Noise;
use super::voronoi::Voronoi;

extern crate web_sys;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen]
pub struct TerrainGenerator {
    #[wasm_bindgen(skip)]
    pub noise: Noise
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

        TerrainGenerator { noise: Noise::new(seed) }
    }


    pub fn fractal_noise (&self, x: f64, y: f64) -> f64 {
        // TODO: move to `noise.rs`
        let force = 0.25; // magic
        let wavyness = 5e-1; // magic

        let theta = self.noise.theta(x * force, y * force);
        let length = self.noise.offset(x * force, y * force);

        let x = x + theta.cos() * length * wavyness;
        let y = y + theta.sin() * length * wavyness;

        self.noise.height(x, y)
    }


    pub fn noise_single (&self, x: f64, y: f64) -> f64 {
        (self.fractal_noise(x, y) + 1.0) / 2.0
    }

    pub fn noise_array (&self, points: Vec<f64>, heights: Option<Vec<f64>>) -> Vec<f64> {
        let heights = match heights {
            None => vec![0.1, 0.2, 0.3],
            Some(heights) => heights,
        };

        heights
            .iter()
            .enumerate()
            .map(|(i, height)| height + self.noise_single(points[i * 2], points[i * 2 + 1]))
            .collect()
    }

    #[wasm_bindgen(js_name = poissonDiscPoints)]
    pub fn poisson_disc_points (&mut self, radius: f64, sea_level: f64, width: f64, height: f64) -> Vec<f64> {
        poisson::disc_sample(radius, sea_level, width, height, self)
    }

    pub fn world (&mut self, radius: f64, sea_level: f64, width: f64, height: f64) -> JsValue {
        let points = self.poisson_disc_points(radius, sea_level, width, height);
        let voronoi = Voronoi::new(points);
        JsValue::from_serde(&voronoi).unwrap()
    }


    // fn check_poisson_sample (row: usize, col: usize, cols: usize, rows: usize, sample: &[f64; 2], grid: &Vec<Vec<[f64; 2]>>, min_offset: f64) -> bool {
    //     let euclidean = |a: &[f64; 2], b: &[f64; 2]| ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt();
    //     'i_loop: for i in ([-1, 0, 1] as [i8; 3]).iter() {
    //         'j_loop: for j in ([-1, 0, 1] as [i8; 3]).iter() {
    //             let neighbor_col = match i {
    //                 -1 => col.checked_sub(1),
    //                 1 => col.checked_add(1),
    //                 _ => Some(col),
    //             };
    //             let neighbor_col = match neighbor_col {
    //                 Some(col) => if col < cols { col } else { continue 'i_loop },
    //                 None => continue 'i_loop,
    //             };
    //
    //             let neighbor_row = match j {
    //                 -1 => row.checked_sub(1),
    //                 1 => row.checked_add(1),
    //                 _ => Some(row),
    //             };
    //             let neighbor_row = match neighbor_row {
    //                 Some(row) => if row < rows { row } else { continue 'j_loop },
    //                 None => continue 'j_loop,
    //             };
    //
    //             let neighbor_i = neighbor_col.wrapping_add(cols * neighbor_row);
    //
    //             for neighbor in grid[neighbor_i].iter() {
    //                 let dist = euclidean(&sample, neighbor);
    //                 if dist < min_offset {
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //     true
    // }

    // fn sample_poisson_points (
    //     &mut self,
    //     k: usize,
    //     size: f64,
    //     width: f64,
    //     height: f64,
    //     min_offset: f64,
    //     point: &[f64; 2],
    //     grid: &mut Vec<Vec<[f64; 2]>>,
    // ) -> Vec<[f64; 2]>
    // {
    //     let mut rng = || self.noise.rng();
    //     let mut new_points: Vec<[f64; 2]> = vec![];
    //
    //     let cols = (width / size) as usize;
    //     let rows = (height / size) as usize;
    //
    //     for _ in 0..k {
    //         // Get a sample at some random angle and distance from `point`
    //         let theta = rng() * PI * 2.0;
    //         let offset = size + rng() * min_offset;
    //         let x = point[0] + theta.cos() * offset;
    //         let y = point[1] + theta.sin() * offset;
    //
    //         // If out of lower bounds, keep looking.
    //         if 0.0 > x || 0.0 > y { continue; }
    //
    //         let sample = [x, y];
    //         let col = (x / size) as usize;
    //         let row = (y / size) as usize;
    //
    //         // If out of upper bounds, keep looking.
    //         if row >= rows || col >= cols { continue; }
    //
    //         if TerrainGenerator::check_poisson_sample(row, col, cols, rows, &sample, &grid, min_offset) == false {
    //             continue; // Check if too close to existing samples. If point is not valid, keep looking.
    //         }
    //         // push sample in
    //         grid[col + row * cols].push(sample);
    //         new_points.push(sample);
    //     }
    //
    //     new_points
    // }

    // fn poisson_add_borders (mut grid: Vec<Vec<[f64; 2]>>, mut active: Vec<[f64; 2]>, mut points: Vec<f64>, size: f64, cols: usize, rows: usize, width: f64, height: f64) -> (Vec<Vec<[f64; 2]>>, Vec<[f64; 2]>, Vec<f64>) {
    //     let size = size / 2.0;
    //     let offset = 5e-2;
    //     let cx = width / 2.0;
    //     let cy = height / 2.0;
    //
    //     // Top
    //     for _x in 0..=(width / size) as usize {
    //         let x = _x as f64 * size;
    //         let y = offset * -(x - cx).abs().cos();
    //         let pos = [x, y];
    //         let i = (x / 2.0 / size) as usize;
    //         grid[i].push(pos);
    //         active.push(pos);
    //         points.extend(pos.iter());
    //     }
    //
    //     // Left
    //     for _y in 0..=(height / size) as usize {
    //         let y = _y as f64 * size;
    //         let x = offset * -(y - cy).abs().cos();
    //         let pos = [x, y];
    //         let j = ((y / 2.0 / size) as usize).min(cols - 1);
    //         grid[j * cols].push(pos);
    //         active.push(pos);
    //         points.extend(pos.iter());
    //     }
    //
    //     // Bottom
    //     for _x in 0..=(width / size) as usize {
    //         let x = _x as f64 * size;
    //         let y = height + offset * (x - cx).abs().cos();
    //         let pos = [x, y];
    //         let i = ((x / 2.0 / size) as usize).min(cols - 1);
    //         grid[i + (rows - 1) * cols].push(pos);
    //         active.push(pos);
    //         points.extend(pos.iter());
    //     }
    //
    //     // Right
    //     for _y in 0..=(height / size) as usize {
    //         let y = _y as f64 * size;
    //         let x = width + offset * (y - cy).abs().cos();
    //         let pos = [x, y];
    //         let j = ((y / 2.0 / size) as usize).min(cols - 1);
    //         grid[cols - 1 + j * cols].push(pos);
    //         active.push(pos);
    //         points.extend(pos.iter());
    //     }
    //     (grid, active, points)
    // }

    // pub fn poisson_disc_points (&mut self, radius: f64, sea_level: f64, width: f64, height: f64) -> Vec<f64> {
    //     poisson::disc_sample(radius, sea_level, width, height, &mut self.noise)
        // let size = radius / (2 as f64).sqrt();
        //
        // let cols = (width / size).floor();
        // let rows = (height / size).floor();
        //
        // let grid: Vec<Vec<[f64; 2]>> = vec![vec![]; (rows * cols) as usize];
        // let active: Vec<[f64; 2]> = Vec::new();
        // let points: Vec<f64> = Vec::new();
        //
        //
        // let destruct = TerrainGenerator::poisson_add_borders(grid, active, points, size, cols as usize, rows as usize, width, height);
        // let mut grid = destruct.0;
        // let mut active = destruct.1;
        // let mut points = destruct.2;
        //
        // let offset_magnitude = |h: f32| if h > sea_level { h } else { 1.0 - h };
        //
        // while active.len() > 0 {
        //     let rand_i = (self.rng.rand::<f64>() * active.len() as f64) as usize;
        //     let point = &active[rand_i];
        //     let min_offset = size * offset_magnitude(self.noise_single(point[0] as f32, point[1] as f32)) as f64;
        //     let new_points = self.sample_poisson_points(30, size, width, height, min_offset, &point, &mut grid);
        //
        //     for sample in new_points.iter() { points.extend(sample.iter()); }
        //     active.extend(new_points.iter());
        //     active.remove(rand_i);
        // }
        //
        // points
    // }
}
