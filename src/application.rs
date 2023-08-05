use std::ffi::CString;

use fermium::{prelude::*, video::SDL_GetWindowSize};

use crate::{mouse::Mouse, cloth::Cloth, renderer::Renderer};

#[derive(Debug)]
pub struct Application {
    renderer: Renderer,
    mouse: Mouse,
    cloth: Cloth,
    is_running: bool,
    last_update_time: u32,
}

impl Application {
    pub fn setup(cloth_width: i32, cloth_height: i32, cloth_spacing: i32) -> Self {
        let mut renderer = Renderer::new();
        let mut mouse = Mouse::new();
        let mut cloth = Cloth::new();

        renderer.setup();

        let is_running = true;

        // let width = cloth_width / cloth_spacing;
        // let height = cloth_height / cloth_spacing;

        // let start_x = renderer.window_width;
        // let start_y = renderer.window_width;

        // cloth.init(width, height, cloth_spacing, start_x, start_y);

        Self {
            renderer,
            mouse,
            cloth,
            is_running,
            last_update_time: unsafe {SDL_GetTicks()}
        }
    }
    
    pub fn is_running(&self) -> bool {self.is_running}
    
    pub fn update(&mut self) {
        let current_time = unsafe { SDL_GetTicks() };
        let delta_time = (current_time - self.last_update_time) / 1000 as u32;

        self.cloth.update(&self.renderer, &self.mouse, delta_time);
        self.last_update_time = current_time;
    }
    
    pub fn render() {}
    
    pub fn destroy() {}
}