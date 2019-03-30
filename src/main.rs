use raylib::Color;

fn main() {
    bench_launch();
}

fn bench_launch() {
    let rl = raylib::init()
        .size(640, 480)
        //.size(1920, 1024)
        .title("Hello, World from RUST")
        .build();

    rl.set_target_fps(60);

    let message = "Hello From RUST and Raylib:";
    let message2 = "What an amazing and promising synergy featuring SlackMagiC !";
    let font = rl.load_font_ex("resources/I-pixel-u.ttf", 80, None);
    let font2 = rl.load_font_ex("resources/ThisSmacky.ttf", 65, None);

    let logo = rl.load_texture("resources/logo.png");

    let position = raylib::Vector2 { x: 20.0, y: 50.0 };
    let position2 = raylib::Vector2 { x: 20.0, y: 70.0 };

    while !rl.window_should_close() {
        rl.begin_drawing();

        rl.clear_background(Color::BEIGE);
        rl.draw_text_ex(&font, &message, position, 20.0, 0.0, Color::DARKBROWN);
        rl.draw_text_ex(&font2, &message2, position2, 13.0, 0.0, Color::BROWN);
        rl.draw_texture(&logo, 20, 90, Color::WHITE);

        rl.draw_fps(500, 0);
        rl.end_drawing();
    }
}

fn proper_launch() {
    with_raylib_handler(|rl| {
        while !rl.window_should_close() {
            update_context();
            draw_screen(&rl);
        }
    });
}

fn update_context() {
    //DO SOME STUFF
}

fn draw_screen(rl: &raylib::RaylibHandle) {
    draw_in_2d(&rl, |rl: &raylib::RaylibHandle| {
        rl.clear_background(Color::BEIGE);
        rl.draw_rectangle(0, 0, 300, 200, Color::ORANGE);
        rl.draw_text("Hello world from RUST !", 12, 12, 20, Color::BROWN);
        rl.draw_text(&rl.get_time().to_string(), 12, 32, 8, Color::RED);
        rl.set_window_title(&rl.get_time().to_string());

        let font = rl.load_font_ex("ThisSmacky.ttf", 16, Some(&[0, 0]));
        let font2 = rl.load_font("I-pixel-u.ttf");
        let position = raylib::Vector2 { x: 20.0, y: 50.0 };

        rl.draw_text_ex(
            &font2,
            "HELLO",
            position,
            (font.baseSize * 4) as f32,
            0.0,
            Color::BLACK,
        );
    });
}

fn with_raylib_handler<F, T>(func: F) -> T
where
    F: FnOnce(&raylib::RaylibHandle) -> T,
{
    let rl = init();
    let ret: T = func(&rl);
    ret
}

fn init() -> raylib::RaylibHandle {
    let rl = raylib::init()
        .size(640, 480)
        //.size(1920, 1024)
        .title("Hello, World from RUST")
        .build();

    rl.hide_cursor();
    rl.set_target_fps(60);
    //rl.toggle_fullscreen();
    rl
}

fn draw_in_2d<F, T>(rl: &raylib::RaylibHandle, func: F) -> T
where
    F: Fn(&raylib::RaylibHandle) -> T,
{
    rl.begin_drawing();
    //Begin DRAWING;
    let ret: T = func(&rl);
    //End DRAWING;
    rl.end_drawing();
    ret
}
