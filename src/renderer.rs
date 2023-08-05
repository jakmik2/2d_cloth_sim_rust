use fermium::prelude::*;

use crate::types::vector2::Vector2;

#[derive(Debug)]
pub struct Renderer {
    pub window_height: i32,
    pub window_width: i32,
    window: *mut SDL_Window,
    renderer: *mut SDL_Renderer
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            window_height: 0,
            window_width: 0,
            window: std::ptr::null_mut(),
            renderer: std::ptr::null_mut()
        }
    }

    pub fn setup(&mut self) {
        unsafe {
            assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0) ;

            let display_mode: *mut SDL_DisplayMode = std::ptr::null_mut();

            // This needs improving
            SDL_GetCurrentDisplayMode(-1, display_mode);

            if !display_mode.is_null() {
                self.window_height = (*display_mode).w;
                self.window_width = (*display_mode).h;
            } else {
                self.window_height = 900;
                self.window_width = 1600;
            }

            println!("{:?} {:?}", self.window_height, self.window_width);

            // Make window
            let window = SDL_CreateWindow(
                b"2dClothSim\0".as_ptr().cast(),
                SDL_WINDOWPOS_CENTERED,
                SDL_WINDOWPOS_CENTERED,
                self.window_width,
                self.window_height,
                (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI).0
            );

            assert!(!window.is_null());

            self.window = window;

            // Make renderer
            let renderer = SDL_CreateRenderer(window, -1, (SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC).0);

            assert!(!renderer.is_null());

            self.renderer = renderer;
        }
    }

    pub fn clear_screen(&self, color: SDL_Color) {
        unsafe {
            SDL_SetRenderDrawColor(self.renderer, color.r, color.g, color.b, color.a);
            SDL_RenderClear(self.renderer);
        }
    }

    pub fn draw_line(&self, p1_pos: Vector2, p2_pos: Vector2, color: SDL_Color) {
        unsafe { 
            SDL_SetRenderDrawColor(self.renderer, color.r, color.g, color.b, color.a);
            SDL_RenderDrawLine(
                self.renderer, 
                p1_pos.x as i32, 
                p1_pos.y as i32, 
                p2_pos.x  as i32, 
                p2_pos.y as i32
            );
        };
    }

    pub fn render(&self) {
        unsafe {
            SDL_RenderPresent(self.renderer);
        }
    }
}