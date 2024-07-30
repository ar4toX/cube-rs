use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(700, 500)
        .title("wine to rev your motor functions")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("Test 1: Successful", 12, 12, 20, Color::WHITE);
    }
}
