pub mod types;
pub mod application;
pub mod mouse;
pub mod cloth;
pub mod renderer;

use fermium::SDL_Quit;

use application::*;

fn main() {
    let mut app = Application::setup(600, 350, 10);

    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }

    unsafe { SDL_Quit() };
}