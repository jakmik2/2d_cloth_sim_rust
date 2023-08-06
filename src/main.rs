pub mod types;
pub mod application;
pub mod mouse;
pub mod cloth;
pub mod renderer;

use std::env;

use fermium::SDL_Quit;

use application::*;

fn main() {
    // Attempt to identify cmd line args
    let args: Vec<String> = env::args().collect();
    
    let mut setup = vec![1200, 700, 10];
    
    dbg!(&args);

    if args.len() == 3 {
        setup = args.clone().iter().map(|x|x.parse().unwrap()).collect();
    }

    dbg!(&setup);

    let mut app = Application::setup(setup[0], setup[1], setup[2]);

    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }

    unsafe { SDL_Quit() };
}