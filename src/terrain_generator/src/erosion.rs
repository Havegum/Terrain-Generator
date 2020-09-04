pub fn get_flux (heights: &Vec<f64>, adjacent: &Vec<Vec<usize>>) -> Vec<f64> {
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

pub fn fill_sinks (heights: Vec<f64>, adjacent: &Vec<Vec<usize>>, sea_level: f64) -> Vec<f64> {
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


pub fn plateau (points: &Vec<f64>, mut heights: Vec<f64>) -> Vec<f64> {
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

pub fn erode (heights: Vec<f64>, adjacent: &Vec<Vec<usize>>, sea_level: f64) -> Vec<f64> {
    let heights = fill_sinks(heights, adjacent, sea_level);

    let flux = get_flux(&heights, adjacent);
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