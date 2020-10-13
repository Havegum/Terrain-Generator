use super::terrain_generator::TerrainGenerator;
use std::f64::consts::PI;

extern crate web_sys;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn disc_sample(radius: f64, sea_level: f64, gen: &mut TerrainGenerator) -> Vec<f64> {
    // Stuff

    let size = radius / (2.0_f64).sqrt();
    let cols = (1.0 / size) as usize;
    let rows = (1.0 / size) as usize;

    let grid: Vec<Vec<[f64; 2]>> = vec![vec![]; rows * cols];
    let active: Vec<[f64; 2]> = Vec::new();
    let points: Vec<f64> = Vec::new();

    let destruct = add_borders(grid, active, points, size, cols, rows);
    let mut grid = destruct.0;
    let mut active = destruct.1;
    let mut points = destruct.2;

    let x = gen.noise.rng();
    let y = gen.noise.rng();
    let sample = [x, y];
    let col = ((x / size) as usize).min(cols - 1);
    let row = ((y / size) as usize).min(rows - 1);
    grid[col + row * cols].push(sample);
    active.push(sample);
    points.extend(sample.iter());

    let offset_magnitude = |h| {
        let n = if h > sea_level { h } else { 1.0 - h };
        n
    };

    while active.len() > 0 {
        let rand_i = (gen.noise.rng() * active.len() as f64) as usize;
        let point = &active[rand_i];
        let min_offset = size * offset_magnitude(gen.noise_single(point[0], point[1]));
        let new_points = sample_poisson_points(30, size, min_offset, &point, &mut grid, gen);

        for sample in new_points.iter() {
            points.extend(sample.iter());
        }
        active.extend(new_points.iter());
        active.remove(rand_i);
    }

    points
}

fn sample_poisson_points(
    k: usize,
    size: f64,
    min_offset: f64,
    point: &[f64; 2],
    grid: &mut Vec<Vec<[f64; 2]>>,
    gen: &mut TerrainGenerator,
) -> Vec<[f64; 2]> {
    let mut new_points: Vec<[f64; 2]> = vec![];

    let cols = (1.0 / size) as usize;
    let rows = (1.0 / size) as usize;

    for _ in 0..k {
        // Get a sample at some random angle and distance from `point`
        let theta = gen.noise.rng() * PI * 2.0;
        let offset = size + gen.noise.rng() * min_offset;
        let x = point[0] + theta.cos() * offset;
        let y = point[1] + theta.sin() * offset;

        // If out of lower bounds, keep looking.
        if 0.0 > x || 0.0 > y {
            continue;
        }

        let sample = [x, y];
        let col = (x / size) as usize;
        let row = (y / size) as usize;

        // If out of upper bounds, keep looking.
        if row >= rows || col >= cols {
            continue;
        }

        if check_sample(row, col, cols, rows, &sample, &grid, min_offset) == false {
            continue; // Check if too close to existing samples. If point is not valid, keep looking.
        }
        // push sample in
        grid[col + row * cols].push(sample);
        new_points.push(sample);
    }

    new_points
}

fn check_sample(
    row: usize,
    col: usize,
    cols: usize,
    rows: usize,
    sample: &[f64; 2],
    grid: &Vec<Vec<[f64; 2]>>,
    min_offset: f64,
) -> bool {
    let euclidean =
        |a: &[f64; 2], b: &[f64; 2]| ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt();

    'i_loop: for i in ([-1, 0, 1] as [i8; 3]).iter() {
        'j_loop: for j in ([-1, 0, 1] as [i8; 3]).iter() {
            let neighbor_col = match i {
                -1 => col.checked_sub(1),
                1 => col.checked_add(1),
                _ => Some(col),
            };
            let neighbor_col = match neighbor_col {
                Some(col) => {
                    if col < cols {
                        col
                    } else {
                        continue 'i_loop;
                    }
                }
                None => continue 'i_loop,
            };

            let neighbor_row = match j {
                -1 => row.checked_sub(1),
                1 => row.checked_add(1),
                _ => Some(row),
            };
            let neighbor_row = match neighbor_row {
                Some(row) => {
                    if row < rows {
                        row
                    } else {
                        continue 'j_loop;
                    }
                }
                None => continue 'j_loop,
            };

            let neighbor_i = neighbor_col.wrapping_add(cols * neighbor_row);

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

fn add_borders(
    mut grid: Vec<Vec<[f64; 2]>>,
    mut active: Vec<[f64; 2]>,
    mut points: Vec<f64>,
    size: f64,
    cols: usize,
    rows: usize,
) -> (Vec<Vec<[f64; 2]>>, Vec<[f64; 2]>, Vec<f64>) {
    let size = size / 2.0;
    let offset = 5e-2;
    let cx = 1.0 / 2.0;
    let cy = 1.0 / 2.0;

    // Top
    for _x in 0..=(1.0 / size) as usize {
        let x = _x as f64 * size;
        let y = offset * -(x - cx).abs().cos();
        let pos = [x, y];
        let i = (x / 2.0 / size) as usize;
        grid[i].push(pos);
        active.push(pos);
        points.extend(pos.iter());
    }

    // Left
    for _y in 0..=(1.0 / size) as usize {
        let y = _y as f64 * size;
        let x = offset * -(y - cy).abs().cos();
        let pos = [x, y];
        let j = ((y / 2.0 / size) as usize).min(cols - 1);
        grid[j * cols].push(pos);
        active.push(pos);
        points.extend(pos.iter());
    }

    // Bottom
    for _x in 0..=(1.0 / size) as usize {
        let x = _x as f64 * size;
        let y = 1.0 + offset * (x - cx).abs().cos();
        let pos = [x, y];
        let i = ((x / 2.0 / size) as usize).min(cols - 1);
        grid[i + (rows - 1) * cols].push(pos);
        active.push(pos);
        points.extend(pos.iter());
    }

    // Right
    for _y in 0..=(1.0 / size) as usize {
        let y = _y as f64 * size;
        let x = 1.0 + offset * (y - cy).abs().cos();
        let pos = [x, y];
        let j = ((y / 2.0 / size) as usize).min(cols - 1);
        grid[cols - 1 + j * cols].push(pos);
        active.push(pos);
        points.extend(pos.iter());
    }

    (grid, active, points)
}
