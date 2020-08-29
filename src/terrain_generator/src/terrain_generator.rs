use wasm_bindgen::prelude::*;

use std::collections::HashSet;

use super::utils;
use super::poisson;
use super::noise::Noise;
use super::voronoi::Voronoi;
use super::erosion::*;

extern crate web_sys;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[derive(Serialize)]
pub struct World {
    // #[wasm_bindgen(readonly)]
    voronoi: Voronoi,
    // #[wasm_bindgen(readonly)]
    heights: Vec<f64>,
    // #[wasm_bindgen(readonly)]
    #[serde(rename="cellHeights")]
    cell_heights: Vec<f64>,
    // #[wasm_bindgen(readonly)]
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
        plateau(&points, heights)
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
        let flux = get_flux(heights, adjacent);
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
        let mut heights = plateau(&voronoi.circumcenters, heights);
        log!(" ·  ✓ and plateaued");

        for _ in 0..10 {
            heights = erode(heights, &voronoi.adjacent, sea_level);
        }
        log!(" ·  ✓ and eroded ×10");

        let cell_heights = TerrainGenerator::get_cell_heights(voronoi.delaunay.points.len() / 2, &heights, &voronoi.voronoi_points);
        log!(" ✓ cell heights calculated");

        let rivers = TerrainGenerator::get_rivers(&heights, &voronoi.adjacent, sea_level, &voronoi.voronoi_cells, &cell_heights);

        let world = World { voronoi, heights, cell_heights, rivers };
        JsValue::from_serde(&world).unwrap()
    }
}
