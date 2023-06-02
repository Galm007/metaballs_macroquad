use macroquad::{prelude::*, rand::gen_range};

mod circle;
use circle::Circle;

mod cell;
mod interpolation;

mod metaballs;
use metaballs::draw_metaballs;

const TOTAL_CIRCLES: usize = 8;
const CELL_SIZE: f32 = 10.0;
const BLOBBYNESS: f32 = 1.4;
const SPEED: f32 = 100.0;
const LINE_CLR: Color = GREEN;
const LINE_THICKNESS: f32 = 1.0;

#[macroquad::main("MetaBalls!")]
async fn main() {
    let mut circles = [Circle::new(0.0, Vec2::ZERO, Vec2::ZERO); TOTAL_CIRCLES];
    for i in 0..TOTAL_CIRCLES {
        let radius = gen_range(60.0, 100.0);
        circles[i].radius = radius;
        circles[i].position = Vec2::new(
            gen_range(radius, screen_width() - radius),
            gen_range(radius, screen_height() - radius),
        );
        circles[i].velocity = Vec2::new(gen_range(-SPEED, SPEED), gen_range(-SPEED, SPEED));
    }

    loop {
        circles.iter_mut().for_each(|c| c.update());

        clear_background(BLACK);
        draw_metaballs(&circles);

        if is_key_down(KeyCode::D) {
            circles
                .iter()
                .for_each(|c| draw_circle_lines(c.position.x, c.position.y, c.radius, 1.0, RED));
        }

        next_frame().await
    }
}
