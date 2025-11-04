use macroquad::prelude::*;

#[macroquad::main("My game")]
async fn main() {
    loop {
        clear_background(DARKGRAY);
        next_frame().await
    }
}
