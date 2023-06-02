use crate::{Circle, cell::cell_value, BLOBBYNESS, TOTAL_CIRCLES};

pub fn interp_x(circles: &[Circle; TOTAL_CIRCLES], min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> f32 {
    let blob = BLOBBYNESS - cell_value(circles, min_x, min_y);
    let celldiff = cell_value(circles, max_x, max_y) - cell_value(circles, min_x, min_y);
    let f = min_x + (max_x - min_x) * (blob / celldiff);
    f.max(min_x).min(max_x)
}

pub fn interp_y(circles: &[Circle; TOTAL_CIRCLES], min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> f32 {
    let blob = BLOBBYNESS - cell_value(circles, min_x, min_y);
    let celldiff = cell_value(circles, max_x, max_y) - cell_value(circles, min_x, min_y);
    let f = min_y + (max_y - min_y) * (blob / celldiff);
    f.max(min_y).min(max_y)
}
