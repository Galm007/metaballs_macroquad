use crate::{Circle, CELL_SIZE, BLOBBYNESS, TOTAL_CIRCLES};

pub fn march_square(circles: &[Circle; TOTAL_CIRCLES], x: f32, y: f32) -> u8 {
    let mut val = 0;

    if cell_value(circles, x, y + CELL_SIZE) >= BLOBBYNESS {
        val |= 1;
    }
    if cell_value(circles, x + CELL_SIZE, y + CELL_SIZE) >= BLOBBYNESS {
        val |= 2;
    }
    if cell_value(circles, x + CELL_SIZE, y) >= BLOBBYNESS {
        val |= 4;
    }
    if cell_value(circles, x, y) >= BLOBBYNESS {
        val |= 8;
    }

    val
}

pub fn cell_value(circles: &[Circle; TOTAL_CIRCLES], x: f32, y: f32) -> f32 {
    let mut sum = 0.0;
    for c in circles.iter() {
        let rad_sq = c.radius * c.radius;
        let diffsq_x = (x - c.position.x) * (x - c.position.x);
        let diffsq_y = (y - c.position.y) * (y - c.position.y);
        sum += rad_sq / (diffsq_x + diffsq_y);
    }
    sum
}
