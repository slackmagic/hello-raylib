use raylib::Color;

fn main() {
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
        rl.draw_fps(500, 0);
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
