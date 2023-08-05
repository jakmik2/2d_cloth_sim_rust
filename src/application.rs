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
        let mouse = Mouse::new();
        let mut cloth = Cloth::new();

        renderer.setup();

        let is_running = true;

        let width = cloth_width / cloth_spacing;
        let height = cloth_height / cloth_spacing;

        let start_x = renderer.window_width - width * cloth_spacing / 2;
        let start_y = renderer.window_width / 10;

        cloth.init(width, height, cloth_spacing, start_x, start_y);

        Self {
            renderer,
            mouse,
            cloth,
            is_running,
            last_update_time: unsafe {SDL_GetTicks()}
        }
    }
    
    pub fn is_running(&self) -> bool {self.is_running}
    
    pub fn input(&mut self) {
        unsafe {
            let mut event = SDL_Event::default();

            while SDL_PollEvent(&mut event as *mut SDL_Event) != 0 {
                match event.type_ {
                    SDL_QUIT => {
                        println!("SDL_QUIT");
                        self.is_running = false;
                        break;
                    }
                    SDL_KEYDOWN => {
                        if event.key.keysym.sym == SDLK_ESCAPE {
                            self.is_running = false;
                        }
                    }
                    SDL_MOUSEMOTION => {
                        self.mouse.update_position(
                            event.motion.x,
                            event.motion.y 
                        );
                    }
                    SDL_MOUSEBUTTONDOWN => {
                        let (mut x, mut y): (i32, i32) = (0,0);
                        SDL_GetMouseState(&mut x as *mut i32, &mut y as *mut i32);
                        self.mouse.update_position(x, y);

                        if !self.mouse.left_button_down && event.button.button == SDL_BUTTON_LEFT as u8 {
                            self.mouse.left_button_down = true;
                        }

                        if !self.mouse.right_button_down && event.button.button == SDL_BUTTON_RIGHT as u8 {
                            self.mouse.right_button_down = true;
                        }
                    }
                    SDL_MOUSEWHEEL => {
                        if event.wheel.y > 0 {
                            self.mouse.increase_cursor_size(10.);
                        } else if event.wheel.y < 0 {
                            self.mouse.increase_cursor_size(-10.);
                        }
                    }
                    _ => ()
                };
            }
        }
    }

    pub fn update(&mut self) {
        let current_time = unsafe { SDL_GetTicks() };
        let delta_time = (current_time - self.last_update_time) / 1000 as u32;

        self.cloth.update(&self.renderer, &self.mouse, delta_time);
        self.last_update_time = current_time;
    }

    pub fn render(&self) {
        self.renderer.clear_screen(SDL_Color { r: 0, g: 8, b: 22, a: 1 });
        self.cloth.draw(&self.renderer);
        self.renderer.render()
    }
    
    pub fn destroy(&self) {
        
    }
}