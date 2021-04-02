pub fn get_flux(heights: &Vec<f64>, adjacent: &Vec<Vec<usize>>) -> Vec<f64> {
    let mut flux = vec![0.0; heights.len()];

    let mut sorted = (0..heights.len()).collect::<Vec<usize>>();
    sorted.sort_unstable_by(|a, b| heights[*a].partial_cmp(&heights[*b]).unwrap().reverse());

    // find downhill for each point.
    for &point in sorted.iter() {
        let lowest_neighbour: usize = *adjacent[point]
            .iter()
            .min_by(|a, b| heights[**a].partial_cmp(&heights[**b]).unwrap())
            .unwrap();

        if adjacent[point].len() > 2 && heights[lowest_neighbour] < heights[point] {
            flux[lowest_neighbour] += flux[point] + 1.0;
        }
    }
    flux
}

pub fn fill_sinks(heights: Vec<f64>, adjacent: &Vec<Vec<usize>>, sea_level: f64) -> Vec<f64> {
    // Mewo implementation details: https://mewo2.com/notes/terrain/
    // Original paper: https://horizon.documentation.ird.fr/exl-doc/pleins_textes/pleins_textes_7/sous_copyright/010031925.pdf
    let epsilon = 1e-5;

    let mut new_heights: Vec<f64> = heights
        .clone()
        .iter()
        .map(|&height| {
            if height > sea_level {
                f64::INFINITY
            } else {
                height
            }
        })
        .collect();

    let mut sorted: Vec<(usize, f64)> = heights.clone().into_iter().enumerate().collect();
    sorted.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let mut changed = true;
    while changed {
        changed = false;

        for &(i, height) in sorted.iter() {
            if new_heights[i] == height {
                continue;
            }

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

pub fn plateau(points: &Vec<f64>, mut heights: Vec<f64>) -> Vec<f64> {
    let plateau_start = 0.45; // Magic
    let plateau_cap = (1. - plateau_start) / 4.; // Magic

    let mut peak_index = 0;
    for (j, &height) in heights.iter().enumerate() {
        if height > heights[peak_index] {
            peak_index = j;
        }
    }
    let peak_x = points[peak_index * 2 + 0];
    let peak_y = points[peak_index * 2 + 1];

    let interpolate = |height: f64| {
        plateau_start
            + (1. - (1. - (height - plateau_start) / (1. - plateau_start)).powi(2)) * plateau_cap
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

pub fn erode(heights: Vec<f64>, adjacent: &Vec<Vec<usize>>, sea_level: f64) -> Vec<f64> {
    // First, smooth out the landscape a bit, and fill sinks
    let heights = smooth(heights, adjacent);
    let heights = fill_sinks(heights, adjacent, sea_level);

    let flux = get_flux(&heights, adjacent);
    let adjacent = adjacent
        .iter()
        .map(|arr| arr.iter().map(|n| heights[*n]).collect::<Vec<f64>>())
        .collect::<Vec<Vec<f64>>>();

    let erosion_rate = 0.015;
    let erosion = |(i, height): (usize, f64)| {
        let point_flux = (flux[i] + 1.).ln();

        let erosion = point_flux * erosion_rate * height;

        if height >= sea_level {
            // Find lowest neighbor.
            let low = adjacent[i]
                .iter()
                .cloned()
                .fold(0. / 0., f64::min)
                .min(height);

            let eroded = height - erosion;
            let alpha = 0.125;

            // If erosion is lower than the lowest neighbor, discount erosion by alpha
            low.max(eroded) * (1. - alpha) + eroded * alpha
        } else {
            height - erosion * 0.25
        }
    };

    let heights = heights
        .into_iter()
        .enumerate()
        .map(erosion)
        .collect::<Vec<f64>>();

    heights
}

pub fn smooth(mut heights: Vec<f64>, adjacent: &Vec<Vec<usize>>) -> Vec<f64> {
    let alpha = 1.;
    let alpha = 0.66;

    for (i, height) in heights
        .clone()
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, f64)>>()
    {
        let sum = adjacent[i].iter().map(|n| heights[*n]).sum::<f64>() + height;

        let mean = sum / (adjacent[i].len() + 1) as f64;

        heights[i] = height * (1. - alpha) + mean * alpha;

        for n in adjacent[i].iter() {
            heights[*n] = heights[*n] * (1. - alpha) + mean * alpha;
        }
    }

    heights
}
