use fermium::{prelude::{SDL_Color, SDLK_SELECT}, SDL_Quit};

use crate::renderer::Renderer;

use super::{particle::ParticleCollection, vector2::Vector2};

#[derive(Debug, PartialEq)]
pub struct Stick {
    pub p1: String,
    pub p2: String,
    pub length: f32,
    pub is_active: bool,
    pub is_selected: bool
}

impl Stick {
    const COLOR: SDL_Color = SDL_Color {r: 255, g: 0, b: 0, a: 1};
    const SELECTED_COLOR: SDL_Color = SDL_Color {r: 204, g: 0, b: 0, a: 1};

    pub fn new(p1: &str, p2: &str, length: f32) -> Self {
        Self {
            p1: p1.to_string(),
            p2: p2.to_string(),
            length,
            is_active: true,
            is_selected: false
        }
    }

    pub fn update(&mut self, points: &mut ParticleCollection) {
        let mut p1 = points.get_particle(self.p1.as_str());
        let mut p2 = points.get_particle(self.p2.as_str());

        self.is_selected = p1.is_selected || p2.is_selected;

        if !self.is_active {
            return;
        }

        let p1_pos = p1.position;
        let p2_pos = p2.position;

        let diff = p1_pos - p2_pos;
        let dist = diff.get_length();
        let diff_factor = (self.length - dist) / dist;

        let offset = diff * diff_factor * 0.5;

        p1.position += offset;
        p2.position -= offset;
        
        points.set_particle(self.p1.as_str(), p1);
        points.set_particle(self.p2.as_str(), p2);
    }

    pub fn draw(&self, renderer: &Renderer, points: &ParticleCollection) {
        if !self.is_active {
            return;
        }

        let p1_pos = points.get_position(self.p1.as_str());
        let p2_pos = points.get_position(self.p2.as_str());

        renderer.draw_line(p1_pos, p2_pos, if self.is_selected {Self::SELECTED_COLOR} else {Self::COLOR})
    }

    pub fn destroy(&mut self) {}
}