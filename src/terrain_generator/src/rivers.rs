use std::collections::HashSet;

use super::erosion::get_flux;

pub fn get_river (
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
            let mut tuple = get_river(&heights, &adjacent, &flux, sea_level, &voronoi_cells, &cell_heights, &mut visited, neighbor, heights[neighbor], river);
            // tuple is (
            //   river: Vec<(index: usize, flux: f64)>,
            //   tributaries: Vec<Vec<(index: usize, flux: f64)>>
            // )
            river = tuple.0;
            tributaries.append(&mut tuple.1);

        } else {
            let mut tuple = get_river(
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

pub fn get_rivers (
    heights: &Vec<f64>,
    adjacent: &Vec<Vec<usize>>,
    sea_level: f64,
    voronoi_cells: &Vec<Vec<usize>>,
    cell_heights: &Vec<f64>
) -> Vec<Vec<(usize, f64)>> {
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
        let mut tuple = get_river(
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
