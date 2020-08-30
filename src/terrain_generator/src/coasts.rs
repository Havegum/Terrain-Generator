use std::collections::HashSet;
use std::iter::FromIterator;

fn get_coast_cells (
    heights: &Vec<f64>,
    neighbors: &Vec<Vec<usize>>,
    sea_level: f64,
) -> Vec<usize> {
    let mut coasts = Vec::new();

    for i in 0..heights.len() {
        if heights[i] >= sea_level {
            let neighbors_sea = neighbors[i].iter().any(|&n| heights[n] < sea_level);
            if neighbors_sea {
                coasts.push(i);
            }
        }
    }

    coasts
}

pub fn get_coast_lines (
    heights: &Vec<f64>,
    neighbors: &Vec<Vec<usize>>,
    voronoi_points: &Vec<Vec<usize>>,
    voronoi_cells: &Vec<Vec<usize>>,
    sea_level: f64,
) -> Vec<(usize, usize)> {
    let coast_cells = get_coast_cells(heights, neighbors, sea_level);
    let mut coast_lines = Vec::new();

    for k in 0..coast_cells.len() {
        let points = &voronoi_points[coast_cells[k]];
        let mut prev = points[points.len() - 1];
        let mut prev_is_border = voronoi_cells[prev].iter().any(|&c| heights[c] < sea_level);

        for i in 0..points.len() {
            let point = points[i];
            let is_border = voronoi_cells[point].iter().any(|&c| heights[c] < sea_level);

            if is_border & prev_is_border {
                let point_neighbors: HashSet<usize> = HashSet::from_iter(voronoi_cells[point]
                    .iter()
                    .filter(|&x| heights[*x] < sea_level)
                    .cloned());
                let prev_neighbors:  HashSet<usize> = HashSet::from_iter(voronoi_cells[prev ]
                    .iter()
                    .filter(|&x| heights[*x] < sea_level)
                    .cloned());

                if !point_neighbors.is_disjoint(&prev_neighbors) {
                    coast_lines.push((point, prev));
                }
            }

            prev = point;
            prev_is_border = is_border;
        }
    }

    coast_lines
}
