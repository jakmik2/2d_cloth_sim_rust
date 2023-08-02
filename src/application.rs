use std::ffi::CString;

use fermium::{prelude::*, video::SDL_GetWindowSize};

use crate::{mouse::Mouse, cloth::Cloth};

#[derive(Debug)]
struct Application {
    renderer: *mut SDL_Renderer,
    window: *mut SDL_Window,
    mouse: Mouse,
    cloth: Cloth,
    is_running: bool,
    last_update_time: u32,
}

impl Application {
    pub fn new() -> Self {
        let title_c_string = CString::new("Application").expect("Failed to make title into c string");
        let window = unsafe { SDL_CreateWindow(
            title_c_string.as_ptr(), 
            SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 
            1200, 900, 
            0)
        };
        let renderer = unsafe { 
            SDL_CreateRenderer(
                window,
                -1, 0
            )
        };
        let mouse = Mouse::new();
        let cloth = Cloth::new();

        Self {
            renderer,
            window,
            mouse,
            cloth,
            is_running: false,
            last_update_time: 0,
        }
    }

    pub fn setup(&mut self, cloth_width: i32, cloth_height: i32, cloth_spacing: i32)  {
        self.is_running = true;

        let width = cloth_width / cloth_spacing;
        let height = cloth_height / cloth_spacing;
    
        let (start_x, start_y) = unsafe {
            let x: *mut c_int;
            let y: *mut c_int;

            SDL_GetWindowSize(self.window, x, y);
            
            (*x, *y)
        };

        self.cloth.init(width, height, cloth_spacing, start_x, start_y)
    }
    
    pub fn is_running(&self) -> bool {self.is_running}
    
    pub fn update(&mut self) {
        let current_time = unsafe { SDL_GetTicks() };
        let delta_time = (current_time - self.last_update_time) / 1000 as u32;

        self.cloth.update(self.renderer, &self.mouse, delta_time);
    }
    
    pub fn render() {}
    
    pub fn destroy() {}
}