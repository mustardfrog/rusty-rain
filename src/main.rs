use macroquad::prelude::*;
pub mod rain;

#[macroquad::main("hello")]
async fn main() {
    let mut r = rain::Rain::new();
    let mut rains = Vec::new();

    for _ in 0..70 {
        rains.push(rain::Rain::new());
    }

    loop {
        clear_background(BLACK);

        for rain in rains.iter_mut() {
            rain.fall(get_frame_time());
            rain.draw();
        }

        r.fall(get_frame_time());
        r.draw();
        next_frame().await
    }
}
