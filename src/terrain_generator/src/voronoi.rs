use delaunator::{triangulate, Point, Triangulation, EMPTY};

#[path = "utils.rs"]
mod utils;

extern crate web_sys;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Implement _init from here:
// https://github.com/d3/d3-delaunay/blob/master/src/voronoi.js

#[derive(Serialize)]
pub struct Voronoi {
    pub circumcenters: Vec<f64>,
    pub delaunay: Delaunay,
    pub adjacent: Vec<Vec<usize>>,
    pub voronoi_triangles: Vec<usize>,
    pub voronoi_points: Vec<Vec<usize>>,
    pub voronoi_cells: Vec<Vec<usize>>,
}

// #[wasm_bindgen]
#[derive(Serialize)]
pub struct Delaunay {
    pub points: Vec<f64>,
    pub hull: Vec<usize>,
    pub inedges: Vec<usize>,
    pub halfedges: Vec<usize>,
    pub triangles: Vec<usize>,
    pub neighbors: Vec<Vec<usize>>,
}

// From `../../terrain.js`
struct Adjacencies {
    adjacent: Vec<Vec<usize>>,
    voronoi_triangles: Vec<usize>,
    voronoi_points: Vec<Vec<usize>>,
    voronoi_cells: Vec<Vec<usize>>,
}

impl Voronoi {
    // Adapted from:
    //     https://github.com/d3/d3-delaunay/blob/master/src/voronoi.js
    //     https://github.com/d3/d3-delaunay/blob/master/src/delaunay.js
    pub fn new(points: Vec<f64> /*, xmin: f64, ymin: f64, xmax: f64, ymax: f64*/) -> Voronoi {
        utils::set_panic_hook();
        let Triangulation {
            triangles,
            halfedges,
            hull,
        } = Voronoi::triangulate(&points);
        let inedges = Voronoi::get_inedges(&points, &halfedges, &triangles);
        let neighbors = Voronoi::get_neighbors(&points, &inedges, &hull, &halfedges, &triangles);
        let circumcenters = Voronoi::get_circumcenters(&points, &triangles);

        let Adjacencies {
            adjacent,
            voronoi_triangles,
            voronoi_points,
            voronoi_cells,
        } = Voronoi::get_adjacencies(&points, &circumcenters, &inedges, &halfedges, &triangles)
            .unwrap();

        let delaunay = Delaunay {
            points,
            hull,
            inedges,
            halfedges,
            triangles,
            neighbors,
        };

        Voronoi {
            circumcenters,
            delaunay,
            adjacent,
            voronoi_triangles,
            voronoi_points,
            voronoi_cells,
        }
    }

    fn triangulate(points: &Vec<f64>) -> Triangulation {
        let struct_points: Vec<&[f64]> = points.chunks_exact(2).collect();
        let struct_points = struct_points
            .iter()
            .map(|p| Point { x: p[0], y: p[1] })
            .collect::<Vec<_>>();

        triangulate(&struct_points).unwrap()
    }

    fn get_inedges(
        points: &Vec<f64>,
        halfedges: &Vec<usize>,
        triangles: &Vec<usize>,
    ) -> Vec<usize> {
        let mut inedges: Vec<usize> = vec![EMPTY; points.len() / 2];
        for e in 0..halfedges.len() {
            let p = triangles[if e % 3 == 2 { e - 2 } else { e + 1 }];
            if halfedges[e] == EMPTY || inedges[p] == EMPTY {
                inedges[p] = e;
            }
        }
        inedges
    }

    fn get_circumcenters(points: &Vec<f64>, triangles: &Vec<usize>) -> Vec<f64> {
        let n = triangles.len();
        let mut circumcenters = vec![0.0; n / 3 * 2];
        let mut i = 0;
        let mut j = 0;

        while i < n {
            let x;
            let y;

            let t1 = triangles[i] * 2;
            let t2 = triangles[i + 1] * 2;
            let t3 = triangles[i + 2] * 2;
            let x1 = points[t1];
            let y1 = points[t1 + 1];
            let x2 = points[t2];
            let y2 = points[t2 + 1];
            let x3 = points[t3];
            let y3 = points[t3 + 1];

            let dx = x2 - x1;
            let dy = y2 - y1;
            let ex = x3 - x1;
            let ey = y3 - y1;
            // let bl = dx * dx + dy * dy;
            // let cl = ex * ex + ey * ey;
            let ab = (dx * ey - dy * ex) * 2.0;

            if ab == 0.0 {
                // degenerate case (collinear diagram)
                x = (x1 + x3) / 2.0 - 1.0e8 * ey;
                y = (y1 + y3) / 2.0 + 1.0e8 * ex;
            } else if ab.abs() < 1e-8 {
                // almost equal points (degenerate triangle)
                x = (x1 + x3) / 2.0;
                y = (y1 + y3) / 2.0;
            } else {
                // let d = 1.0 / ab;
                // x = x1 + (ey * bl - dy * cl) * d;
                // y = y1 + (dx * cl - ex * bl) * d;
                x = (x1 + x2 + x3) / 3.0;
                y = (y1 + y2 + y3) / 3.0;
            }
            circumcenters[j] = x;
            circumcenters[j + 1] = y;
            i += 3;
            j += 2;
        }
        // TODO: exterior hull?
        circumcenters
    }

    fn get_adjacencies(
        points: &Vec<f64>,
        circumcenters: &Vec<f64>,
        inedges: &Vec<usize>,
        halfedges: &Vec<usize>,
        triangles: &Vec<usize>,
    ) -> Result<Adjacencies, String> {
        let mut adjacent = vec![Vec::new(); circumcenters.len() / 2];
        let mut voronoi_triangles = Vec::new();
        let mut voronoi_points = vec![Vec::new(); points.len() / 2];
        let mut voronoi_cells = vec![Vec::new(); circumcenters.len() / 2];

        for i in 0..inedges.len() {
            let e0 = inedges[i];
            if e0 == EMPTY {
                return Err("Coincident point".to_string());
            } // coincident point
            let mut e = e0;
            let mut t;
            let mut previous_t = EMPTY;

            loop {
                t = e / 3;
                voronoi_cells[t].push(i);
                voronoi_points[i].push(t);

                // Index `t` is neighbour of the previous `t`
                if previous_t != EMPTY {
                    if !adjacent[t].contains(&previous_t) {
                        adjacent[t].push(previous_t);
                    }
                    if !adjacent[previous_t].contains(&t) {
                        adjacent[previous_t].push(t);
                    }
                    voronoi_triangles.extend([i, t, previous_t].iter());
                }
                previous_t = t;

                e = if e % 3 == 2 { e - 2 } else { e + 1 };
                if triangles[e] != i {
                    break;
                } // bad triangulation
                e = halfedges[e];

                if e == e0 || e == EMPTY {
                    break;
                }
            }

            voronoi_triangles.extend([i, e / 3, previous_t].iter());
        }

        Ok(Adjacencies {
            adjacent,
            voronoi_triangles,
            voronoi_points,
            voronoi_cells,
        })
    }

    fn get_neighbors(
        points: &Vec<f64>,
        inedges: &Vec<usize>,
        hull: &Vec<usize>,
        halfedges: &Vec<usize>,
        triangles: &Vec<usize>,
    ) -> Vec<Vec<usize>> {
        let mut neighbors = vec![Vec::new(); points.len() / 2];
        let mut hull_index = vec![EMPTY; points.len() / 2];
        for i in 0..hull.len() {
            hull_index[hull[i]] = i;
        }

        for i in 0..points.len() / 2 {
            let e0 = inedges[i];
            let mut e = e0;
            let mut p0;

            loop {
                p0 = triangles[e];
                neighbors[i].push(p0);
                e = if e % 3 == 2 { e - 2 } else { e + 1 };

                e = halfedges[e];
                if e == EMPTY {
                    let p = hull[(hull_index[i] + 1) % hull.len()];
                    if p != p0 {
                        neighbors[i].push(p);
                    }
                    break;
                }
                if e == e0 {
                    break;
                }
            }
        }
        neighbors
    }
}
