use super::erosion::get_flux;

type River = Vec<(usize, f64)>;

pub fn get_river(
    heights: &Vec<f64>,
    adjacent: &Vec<Vec<usize>>,
    flux: &Vec<f64>,
    sea_level: f64,
    voronoi_cells: &Vec<Vec<usize>>,
    cell_heights: &Vec<f64>,
    mut visited: &mut [bool],
    i: usize,
    mut river: Vec<(usize, f64)>,
) -> (River, Vec<River>) {
    visited[i] = true; // Whatever happens next, mark this node as visited
    let height = heights[i];

    if height < sea_level {
        // If we're undersea, check if at least two adjacent cells are land
        let cells = &voronoi_cells[i];
        let num_adjacent = cells
            .iter()
            .filter(|cell| cell_heights[**cell] > sea_level)
            .count();

        // If not, return empty
        if num_adjacent < 2 {
            return (river, Vec::new());
        }
    }

    river.push((i, flux[i])); // Include this node to the main river

    let mut tributaries: Vec<River> = Vec::new(); // Find rivers that run into this one.
    let mut main_branch_found = false;

    // Check all neighbors by reverse flux order
    let mut neighbors = adjacent[i].clone();
    neighbors.sort_unstable_by(|&a, &b| flux[a].partial_cmp(&flux[b]).unwrap().reverse());

    for neighbor in neighbors {
        if visited[neighbor] {
            continue;
        }
        if adjacent[neighbor].iter().any(|n| heights[*n] < height) {
            continue; // if there exists a lower neighbor for this neighbor, skip
        }

        // Otherwise, continue recursion for either main branch or tributaries
        if !main_branch_found {
            main_branch_found = true;
            let (new_river, mut new_tributaries) = get_river(
                &heights,
                &adjacent,
                &flux,
                sea_level,
                &voronoi_cells,
                &cell_heights,
                &mut visited,
                neighbor,
                river,
            );
            river = new_river;
            tributaries.append(&mut new_tributaries);
        } else {
            let (new_river, mut new_tributaries) = get_river(
                &heights,
                &adjacent,
                &flux,
                sea_level,
                &voronoi_cells,
                &cell_heights,
                &mut visited,
                neighbor,
                vec![(i, flux[i])],
            );
            tributaries.push(new_river);
            tributaries.append(&mut new_tributaries);
        }
    }

    (river, tributaries)
}

pub fn get_rivers(
    heights: &Vec<f64>,
    adjacent: &Vec<Vec<usize>>,
    sea_level: f64,
    voronoi_cells: &Vec<Vec<usize>>,
    cell_heights: &Vec<f64>,
) -> Vec<River> {
    let flux = get_flux(heights, adjacent);

    let mut points_by_height = (0..heights.len()).collect::<Vec<usize>>();
    points_by_height.sort_unstable_by(|a, b| heights[*a].partial_cmp(&heights[*b]).unwrap());

    let mut visited = vec![false; heights.len()];
    let mut rivers: Vec<River> = Vec::new();

    for &i in points_by_height.iter() {
        if visited[i] {
            continue;
        }
        // Might want to continue here if height < sea_level.
        let (new_river, mut new_tributaries) = get_river(
            &heights,
            &adjacent,
            &flux,
            sea_level,
            &voronoi_cells,
            &cell_heights,
            &mut visited,
            i,
            Vec::new(),
        );
        rivers.push(new_river);
        rivers.append(&mut new_tributaries);
    }

    rivers
        .into_iter()
        .filter(|r| r.len() > 1)
        .collect::<Vec<River>>()
}
