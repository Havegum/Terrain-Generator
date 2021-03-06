use wasm_bindgen::prelude::*;

use super::coasts::*;
use super::erosion::*;
use super::noise::Noise;
use super::poisson;
use super::rivers::*;
use super::utils;
use super::voronoi::Voronoi;

extern crate web_sys;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        if cfg![target = "wasm32-unknown-unknown"] {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
    }
}

#[wasm_bindgen(readonly)]
#[derive(Serialize, Debug, PartialEq)]
pub struct World {
    voronoi: Voronoi,
    heights: Vec<f64>,

    #[serde(rename = "cellHeights")]
    cell_heights: Vec<f64>,
    rivers: Vec<Vec<(usize, f64)>>,

    #[serde(rename = "coastLines")]
    coast_lines: Vec<(usize, usize)>,
}

#[wasm_bindgen]
impl World {
    pub fn as_js_value(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}

#[wasm_bindgen]
pub struct TerrainGenerator {
    #[wasm_bindgen(skip)]
    pub noise: Noise,
}

#[wasm_bindgen]
impl TerrainGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: Option<u32>) -> TerrainGenerator {
        if cfg![target = "wasm32-unknown-unknown"] {
            utils::set_panic_hook();
        }

        let seed = match seed {
            None => 123456 as u64,
            Some(seed) => seed as u64,
        };

        TerrainGenerator {
            noise: Noise::new(seed),
        }
    }

    pub fn noise_single(&self, x: f64, y: f64) -> f64 {
        (self.noise.fractal_noise(x, y) + 1.) / 2.
    }

    #[wasm_bindgen(js_name = "heightmap")]
    pub fn heightmap_js(&self, points: Vec<f64>, heights: Option<Vec<f64>>) -> Vec<f64> {
        let heights = self.noise_array(&points, heights);
        plateau(&points, heights)
    }

    fn noise_array(&self, points: &Vec<f64>, heights: Option<Vec<f64>>) -> Vec<f64> {
        let heights = match heights {
            None => vec![0.; points.len() / 2],
            Some(heights) => heights,
        };

        let noise = |(i, height)| height + self.noise_single(points[i * 2], points[i * 2 + 1]);

        heights.iter().enumerate().map(noise).collect()
    }

    fn get_cell_heights(
        n: usize,
        heights: &Vec<f64>,
        voronoi_points: &Vec<Vec<usize>>,
    ) -> Vec<f64> {
        let mut cell_heights = vec![0.; n];
        for i in 0..n {
            let points = &voronoi_points[i];
            cell_heights[i] = points.iter().map(|&n| heights[n]).sum::<f64>() / points.len() as f64;
        }
        cell_heights
    }

    pub fn world(&mut self, radius: f64, sea_level: f64) -> World {
        log!("`world` called");
        let points = poisson::disc_sample(radius, sea_level, self);
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
        let cell_heights = TerrainGenerator::get_cell_heights(
            voronoi.delaunay.points.len() / 2,
            &heights,
            &voronoi.voronoi_points,
        );

        let rivers = get_rivers(
            &heights,
            &voronoi.adjacent,
            sea_level,
            &voronoi.voronoi_cells,
            &cell_heights,
        );
        log!(" ✓ rivers flowed");

        let coast_lines = get_coast_lines(
            &cell_heights,
            &voronoi.delaunay.neighbors,
            &voronoi.voronoi_points,
            &voronoi.voronoi_cells,
            sea_level,
        );
        log!(" ✓ coasts lines carved");

        World {
            voronoi,
            heights,
            cell_heights,
            rivers,
            coast_lines,
        }
    }
}
