use crate::{cell::*, circle::Circle, interpolation::*, CELL_SIZE, LINE_CLR, LINE_THICKNESS, TOTAL_CIRCLES};
use macroquad::prelude::*;

pub fn draw_metaballs(circles: &[Circle; TOTAL_CIRCLES]) {
    for i in (0..screen_height() as usize).step_by(CELL_SIZE as usize) {
        for j in (0..screen_width() as usize).step_by(CELL_SIZE as usize) {
            draw_cell(circles, i, j);
        }
    }
}

fn draw_cell(circles: &[Circle; TOTAL_CIRCLES], cell_y: usize, cell_x: usize) {
    let y = cell_y as f32;
    let x = cell_x as f32;

    match march_square(circles, x, y) {
        1 => {
            let ypos = interp_y(circles, x, y, x, y + CELL_SIZE);
            let xpos = interp_x(circles, x, y + CELL_SIZE, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(x, ypos, xpos, y + CELL_SIZE, LINE_THICKNESS, LINE_CLR);
        }
        2 => {
            let xpos = interp_x(circles, x, y + CELL_SIZE, x + CELL_SIZE, y + CELL_SIZE);
            let ypos = interp_y(circles, x + CELL_SIZE, y, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(
                xpos,
                y + CELL_SIZE,
                x + CELL_SIZE,
                ypos,
                LINE_THICKNESS,
                LINE_CLR,
            );
        }
        3 => {
            let ypos = interp_y(circles, x, y, x, y + CELL_SIZE);
            let ypos2 = interp_y(circles, x + CELL_SIZE, y, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(x, ypos, x + CELL_SIZE, ypos2, LINE_THICKNESS, LINE_CLR);
        }
        4 => {
            let xpos = interp_x(circles, x, y, x + CELL_SIZE, y);
            let ypos = interp_y(circles, x + CELL_SIZE, y, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(xpos, y, x + CELL_SIZE, ypos, LINE_THICKNESS, LINE_CLR);
        }
        5 => {
            let ypos = interp_y(circles, x, y, x, y + CELL_SIZE);
            let xpos = interp_x(circles, x, y, x + CELL_SIZE, y);
            draw_line(x, ypos, xpos, y, LINE_THICKNESS, LINE_CLR);
            let xpos = interp_x(circles, x, y + CELL_SIZE, x + CELL_SIZE, y + CELL_SIZE);
            let ypos = interp_y(circles, x + CELL_SIZE, y, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(
                xpos,
                y + CELL_SIZE,
                x + CELL_SIZE,
                ypos,
                LINE_THICKNESS,
                LINE_CLR,
            );
        }
        6 => {
            let xpos = interp_x(circles, x, y, x + CELL_SIZE, y);
            let xpos2 = interp_x(circles, x, y + CELL_SIZE, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(xpos, y, xpos2, y + CELL_SIZE, LINE_THICKNESS, LINE_CLR);
        }
        7 => {
            let ypos = interp_y(circles, x, y, x, y + CELL_SIZE);
            let xpos = interp_x(circles, x, y, x + CELL_SIZE, y);
            draw_line(x, ypos, xpos, y, LINE_THICKNESS, LINE_CLR);
        }
        8 => {
            let ypos = interp_y(circles, x, y, x, y + CELL_SIZE);
            let xpos = interp_x(circles, x, y, x + CELL_SIZE, y);
            draw_line(x, ypos, xpos, y, LINE_THICKNESS, LINE_CLR);
        }
        9 => {
            let xpos = interp_x(circles, x, y, x + CELL_SIZE, y);
            let xpos2 = interp_x(circles, x, y + CELL_SIZE, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(xpos, y, xpos2, y + CELL_SIZE, LINE_THICKNESS, LINE_CLR);
        }
        10 => {
            let ypos = interp_y(circles, x, y, x, y + CELL_SIZE);
            let xpos = interp_x(circles, x, y + CELL_SIZE, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(x, ypos, xpos, y + CELL_SIZE, LINE_THICKNESS, LINE_CLR);
            let xpos = interp_x(circles, x, y, x + CELL_SIZE, y);
            let ypos = interp_y(circles, x + CELL_SIZE, y, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(xpos, y, x + CELL_SIZE, ypos, LINE_THICKNESS, LINE_CLR);
        }
        11 => {
            let xpos = interp_x(circles, x, y, x + CELL_SIZE, y);
            let ypos = interp_y(circles, x + CELL_SIZE, y, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(xpos, y, x + CELL_SIZE, ypos, LINE_THICKNESS, LINE_CLR);
        }
        12 => {
            let ypos = interp_y(circles, x, y, x, y + CELL_SIZE);
            let ypos2 = interp_y(circles, x + CELL_SIZE, y, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(x, ypos, x + CELL_SIZE, ypos2, LINE_THICKNESS, LINE_CLR);
        }
        13 => {
            let xpos = interp_x(circles, x, y + CELL_SIZE, x + CELL_SIZE, y + CELL_SIZE);
            let ypos = interp_y(circles, x + CELL_SIZE, y, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(
                xpos,
                y + CELL_SIZE,
                x + CELL_SIZE,
                ypos,
                LINE_THICKNESS,
                LINE_CLR,
            );
        }
        14 => {
            let ypos = interp_y(circles, x, y, x, y + CELL_SIZE);
            let xpos = interp_x(circles, x, y + CELL_SIZE, x + CELL_SIZE, y + CELL_SIZE);
            draw_line(x, ypos, xpos, y + CELL_SIZE, LINE_THICKNESS, LINE_CLR);
        }
        _ => {}
    }
}
