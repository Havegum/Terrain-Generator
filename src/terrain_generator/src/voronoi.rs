use delaunator::{EMPTY, Point, triangulate, Triangulation};

// Implement _init from here:
// https://github.com/d3/d3-delaunay/blob/master/src/voronoi.js


pub struct Voronoi {
    pub circumcenters: Vec<f64>,
    pub delaunay: Delaunay,
}

pub struct Delaunay {
    pub points: Vec<f64>,
    pub hull: Vec<usize>,
    pub inedges: Vec<usize>,
    pub halfedges: Vec<usize>,
    pub triangles: Vec<usize>,
}

impl Voronoi {
    pub fn new (points: Vec<f64>, xmin: f64, ymin: f64, xmax: f64, ymax: f64) -> Voronoi {
        let struct_points = &points
            .chunks_exact(2)
            .map(|p| Point { x: p[0], y: p[1] })
            .collect::<Vec<_>>();
        let Triangulation { triangles, halfedges, hull } = triangulate(struct_points).unwrap();

        // From https://github.com/d3/d3-delaunay/blob/master/src/delaunay.js
        let mut inedges: Vec<usize> = vec![EMPTY; points.len() / 2];
        for e in 0..halfedges.len() - 1 {
            let p = triangles[if e % 3 == 2 { e - 2 } else { e + 1 }];
            if halfedges[e] == EMPTY || inedges[p] == EMPTY { inedges[p] = e; }
        }

        let mut circumcenters = vec![0.0; triangles.len() / 3 * 2];
        {
            let mut i = 0;
            let mut j = 0;
            let n = triangles.len();

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
                let bl = dx * dx + dy * dy;
                let cl = ex * ex + ey * ey;
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
                    let d = 1.0 / ab;
                    x = x1 + (ey * bl - dy * cl) * d;
                    y = y1 + (dx * cl - ex * bl) * d;
                }
                circumcenters[j] = x;
                circumcenters[j + 1] = y;
                i += 3;
                j += 2;
            }
            // TODO: exterior hull?
        }

        let delaunay = Delaunay {
            points,
            hull,
            inedges,
            halfedges,
            triangles,
        };

        Voronoi {
            circumcenters,
            delaunay
        }
    }

    // fn get_circumcenters() -> Vec<f64> {
    //
    // }

    // pub fn getNeighbors (i: usize) -> Vec<usize>;
}
