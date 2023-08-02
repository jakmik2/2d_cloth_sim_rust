use fermium::prelude::*;

use crate::{mouse::Mouse, types::{vector2::Vector2, particle::*, stick::Stick}};

#[derive(Debug)]
pub struct Cloth {
    gravity: Vector2,
    drag: f32,
    elasticity: f32,
    points: ParticleCollection,
    sticks: Vec<Stick>,
}

impl Cloth {
    pub fn new() -> Self {        
        Self {
            gravity: Vector2 {x: 0., y: 981.},
            drag: 0.01,
            elasticity: 10.,
            points: ParticleCollection::new_empty(),
            sticks: Vec::new()
        }
    }

    pub fn init(&mut self, width: u32, height: u32, spacing: u32, start_x: u32, start_y: u32) {   
        for y in 0..height {
            for x in 0..width {
                let point = Particle::at_position(Vector2 {x: (start_x + x * spacing) as f32, y: (start_y + y * spacing) as f32});

                if x != 0 {
                    let left_point = self.points.get_particle((self.points.size() - 1).to_string().as_str());
                    let stick = Stick::new(
                        self.points.size().to_string().as_str(), 
                        (self.points.size() - 1).to_string().as_str(),
                        point.position.get_distance_to(left_point.position)
                    );
                    self.sticks.push(stick);
                }

                if y != 0 {
                    let up_point = self.points.get_particle((x + (y - 1) * (width + 1)).to_string().as_str());
                    let stick = Stick::new(
                        self.points.size().to_string().as_str(),
                        (x + (y - 1) * (width + 1)).to_string().as_str(),
                        point.position.get_distance_to(up_point.position)
                    );
                    self.sticks.push(stick);
                }

                if y ==0 && x % 2 == 0 {
                    point.pin();
                }

                self.points.set_particle(self.points.size().to_string().as_str(), point);
            }
        }
    }
        

    pub fn update(&mut self, renderer: *mut SDL_Renderer, mouse: &Mouse, delta_time: u32) {
        // TODO
    }

    pub fn draw(&mut self, renderer: &SDL_Renderer, mouse: &Mouse, delta_time: u32) {
        // TODO
    }
}