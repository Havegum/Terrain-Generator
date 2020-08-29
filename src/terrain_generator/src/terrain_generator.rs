use wasm_bindgen::prelude::*;

use std::collections::HashSet;

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


#[derive(Serialize)]
pub struct World {
    voronoi: Voronoi,
    heights: Vec<f64>,
    #[serde(rename="cellHeights")]
    cell_heights: Vec<f64>,
    rivers: Vec<Vec<(usize, f64)>>,

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

    pub fn noise_single (&self, x: f64, y: f64) -> f64 {
        (self.noise.fractal_noise(x, y) + 1.) / 2.
    }

    #[wasm_bindgen(js_name="heightmap")]
    pub fn heightmap_js (&self, points: Vec<f64>, heights: Option<Vec<f64>>) -> Vec<f64> {
        let heights = self.noise_array(&points, heights);
        TerrainGenerator::plateau(&points, heights)
    }

    fn noise_array (&self, points: &Vec<f64>, heights: Option<Vec<f64>>) -> Vec<f64> {
        let heights = match heights {
            None => vec![0.; points.len() / 2],
            Some(heights) => heights,
        };

        let noise = |(i, height)| height + self.noise_single(points[i * 2], points[i * 2 + 1]);

        heights
            .iter()
            .enumerate()
            .map(noise)
            .collect()
    }


    fn plateau (points: &Vec<f64>, mut heights: Vec<f64>) -> Vec<f64> {
        let plateau_start = 0.45; // Magic
        let plateau_cap = (1. - plateau_start) / 4.; // Magic

        let mut peak_index = 0;
        for (j, &height) in heights.iter().enumerate() {
            if height > heights[peak_index] { peak_index = j; }
        }
        let peak_x = points[peak_index * 2 + 0];
        let peak_y = points[peak_index * 2 + 1];

        let interpolate = |i: f64| {
            plateau_start + (1. - (1. - (i - plateau_start) / (1. - plateau_start)).powi(2)) * plateau_cap
        };

        for i in 0..heights.len() {
            let height = heights[i];

            let x = points[i * 2 + 0];
            let y = points[i * 2 + 1];

            let distance_to_peak = ((x - peak_x).hypot(y - peak_y).min(0.5) / 0.5).powi(2);
            heights[i] = (1. - distance_to_peak) * height + distance_to_peak * interpolate(height);
        }

        heights
    }

    fn erode (heights: Vec<f64>, adjacent: &Vec<Vec<usize>>, sea_level: f64) -> Vec<f64> {
        let heights = TerrainGenerator::fill_sinks(heights, adjacent, sea_level);

        let flux = TerrainGenerator::get_flux(&heights, adjacent);
        let n = heights.len() as f64;

        let erosion_rate = 0.0125;
        let flux_exponent = 1e3 as i32;

        let erosion = |(i, height): (usize, f64)| {
            let underwater_discount = if height < sea_level
                { 1e4_f64.powf(height - sea_level) } else { 1. };
            let point_flux = 1. - (1. - flux[i] / n).powi(flux_exponent);
            height - point_flux * point_flux * erosion_rate * underwater_discount
        };

        heights
            .into_iter()
            .enumerate()
            .map(erosion)
            .collect::<Vec<f64>>()
    }


    fn fill_sinks (heights: Vec<f64>, adjacent: &Vec<Vec<usize>>, sea_level: f64) -> Vec<f64> {
        // Mewo implementation details: https://mewo2.com/notes/terrain/
        // Original paper: https://horizon.documentation.ird.fr/exl-doc/pleins_textes/pleins_textes_7/sous_copyright/010031925.pdf
        let epsilon = 1e-5;

        let mut new_heights: Vec<f64> = heights
            .clone()
            .iter()
            .map(|&height| if height > sea_level { f64::INFINITY } else { height })
            .collect();

        let mut sorted: Vec<(usize, f64)> = heights
            .clone()
            .into_iter()
            .enumerate()
            .collect();
        sorted.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

        let mut changed = true;
        while changed {
            changed = false;

            for &(i, height) in sorted.iter() {
                if new_heights[i] == height { continue; }

                let neighbors = &adjacent[i];
                for &neighbor in neighbors.iter() {
                    let other = new_heights[neighbor] + epsilon;

                    if height >= other {
                        new_heights[i] = height;
                        changed = true;
                        break;
                    }

                    if new_heights[i] > other && other > height {
                        new_heights[i] = other;
                        changed = true;
                    }
                }
            }
        }

        new_heights
    }

    fn get_flux (heights: &Vec<f64>, adjacent: &Vec<Vec<usize>>) -> Vec<f64> {
        let mut flux = vec![0.; heights.len()];

        let mut sorted = heights
            .clone()
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, f64)>>();
        sorted.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

        // find downhill for each point.
        for &(k, height) in sorted.iter().rev() {
            let neighbors = &adjacent[k];

            let mut lowest:Option<usize> = None;
            for &n in neighbors.iter() {
                if heights[n] < height {
                    lowest = Some(match lowest {
                        Some(low) => if heights[n] < heights[low] { n } else { low },
                        None => n,
                    });
                }
            }
            if let Some(neighbor) = lowest {
                flux[neighbor] = flux[neighbor] + flux[k] + 1.;
            }
        }
        flux
    }

    fn get_cell_heights (n: usize, heights: &Vec<f64>, voronoi_points: &Vec<Vec<usize>>) -> Vec<f64> {
        let mut cell_heights = vec![0.; n];
        for i in 0..n {
            let points = &voronoi_points[i];
            cell_heights[i] = points
                .iter()
                .map(|&n| heights[n])
                .sum::<f64>() / points.len() as f64;
        }
        cell_heights
    }

    fn get_river (
        heights: &Vec<f64>,
        adjacent: &Vec<Vec<usize>>,
        flux: &Vec<f64>,
        sea_level: f64,
        voronoi_cells: &Vec<Vec<usize>>,
        cell_heights: &Vec<f64>,
        mut visited: &mut HashSet<usize>,
        i: usize,
        height: f64,
        mut river: Vec<(usize, f64)>
    ) -> (Vec<(usize, f64)>, Vec<Vec<(usize, f64)>>)
    {
        visited.insert(i); // Whatever happens next, mark this node as visited


        if height < sea_level {
             // If we're undersea, check if at least two adjacent cells are land
            let cells = &voronoi_cells[i];
            let adjacent = cells
                .into_iter()
                .map(|cell| cell_heights[*cell])
                .filter(|height| *height > sea_level)
                .collect::<Vec<f64>>()
                .len();

            // If not, return empty
            if adjacent < 2 {
                return (river, Vec::new());
            }
        }

        river.push((i, flux[i])); // Include this node to the main river
        let mut tributaries: Vec<Vec<(usize, f64)>> = Vec::new();  // Find rivers that run into this one.
        let mut main_branch_found = false;

        // Check all neighbors by flux order
        let mut neighbors = adjacent[i].clone();
        neighbors.sort_unstable_by(|&a, &b| flux[a].partial_cmp(&flux[b]).unwrap());
        for &neighbor in neighbors.iter().rev() {
            if visited.contains(&neighbor) { continue }
            if height > adjacent[neighbor].iter().map(|&a| heights[a]).fold(f64::INFINITY, |a, b| a.min(b)) {
                continue // if there exists a lower neighbor for this neighbor, skip
            }

            // Otherwise, continue recursion for either main branch or tributaries
            if !main_branch_found {
                main_branch_found = true;
                let mut tuple = TerrainGenerator::get_river(&heights, &adjacent, &flux, sea_level, &voronoi_cells, &cell_heights, &mut visited, neighbor, heights[neighbor], river);
                // tuple is (
                //   river: Vec<(index: usize, flux: f64)>,
                //   tributaries: Vec<Vec<(index: usize, flux: f64)>>
                // )
                river = tuple.0;
                tributaries.append(&mut tuple.1);

            } else {
                let mut tuple = TerrainGenerator::get_river(
                    &heights,
                    &adjacent,
                    &flux,
                    sea_level,
                    &voronoi_cells,
                    &cell_heights,
                    &mut visited,
                    neighbor,
                    heights[neighbor],
                    vec![(i, flux[i])]);
                tributaries.push(tuple.0);
                tributaries.append(&mut tuple.1);

            }
        }

        (river, tributaries)
    }

    fn get_rivers (heights: &Vec<f64>, adjacent: &Vec<Vec<usize>>, sea_level: f64, voronoi_cells: &Vec<Vec<usize>>, cell_heights: &Vec<f64>) -> Vec<Vec<(usize, f64)>> {
        let flux = TerrainGenerator::get_flux(heights, adjacent);
        let mut sorted = heights
            .clone()
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, f64)>>();
        sorted.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

        let mut visited = HashSet::new();
        let mut rivers: Vec<Vec<(usize, f64)>> = Vec::new();

        for &(i, height) in sorted.iter() {
            if visited.contains(&i) { continue }
            // Might want to continue here if height < sea_level.
            let mut tuple = TerrainGenerator::get_river(
                &heights,
                &adjacent,
                &flux,
                sea_level,
                &voronoi_cells,
                &cell_heights,
                &mut visited,
                i,
                height,
                Vec::new()
            );
            rivers.push(tuple.0);
            rivers.append(&mut tuple.1);
        }

        rivers.into_iter().filter(|r| r.len() > 1).collect::<Vec<Vec<(usize, f64)>>>()
    }

    pub fn world (&mut self, radius: f64, sea_level: f64, width: f64, height: f64) -> JsValue {
        log!("`world` called");
        let points = poisson::disc_sample(radius, sea_level, width, height, self);
        log!(" ✓ points poissoned");
        let voronoi = Voronoi::new(points);
        log!(" ✓ voronoi triangulated");

        let heights = self.noise_array(&voronoi.circumcenters, None);
        log!(" ✓ heights noised");
        let mut heights = TerrainGenerator::plateau(&voronoi.circumcenters, heights);
        log!(" ·  ✓ and plateaued");

        for _ in 0..10 {
            heights = TerrainGenerator::erode(heights, &voronoi.adjacent, sea_level);
        }
        log!(" ·  ✓ and eroded ×10");

        let cell_heights = TerrainGenerator::get_cell_heights(voronoi.delaunay.points.len() / 2, &heights, &voronoi.voronoi_points);
        log!(" ✓ cell heights calculated");

        let rivers = TerrainGenerator::get_rivers(&heights, &voronoi.adjacent, sea_level, &voronoi.voronoi_cells, &cell_heights);

        let world = World { voronoi, heights, cell_heights, rivers };
        JsValue::from_serde(&world).unwrap()
    }
}
