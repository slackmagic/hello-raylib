use raylib::Color;

fn main() {
    println!("Hello, world!");
    let rl = raylib::init()
        .size(640, 480)
        .title("Hello, World from RUST")
        .build();

    while !rl.window_should_close() {
        rl.begin_drawing();

        rl.clear_background(Color::BEIGE);
        rl.draw_text("Hello world from RUST !", 12, 12, 20, Color::BROWN);

        rl.end_drawing();
    }
}
