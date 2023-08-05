pub mod types;
pub mod application;
pub mod mouse;
pub mod cloth;
pub mod renderer;

use fermium::SDL_Quit;
// use macroquad::prelude::*;
use application::*;

fn main() {
    let mut app = Application::setup(1200,320, 20);
    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }
    unsafe { SDL_Quit() };
}