use std::{sync::Arc, collections::{HashMap, hash_map::Keys}};

use fermium::prelude::SDL_Color;

use crate::{types::vector2::Vector2, mouse::Mouse, renderer::Renderer};

// A basic particle containing Position, Velocity and Mass
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Particle {
    pub position: Vector2,
    pub prev_pos: Vector2,
    pub init_pos: Vector2,
    pub velocity: Vector2,
    pub mass: f32,
    pub is_pinned: bool,
    pub is_selected: bool
}

impl Particle {
    pub fn new() -> Self {
        Particle { 
            position: Vector2::UNIT_VECTOR,
            prev_pos: Vector2::UNIT_VECTOR,
            init_pos: Vector2::UNIT_VECTOR,
            velocity: Vector2::UNIT_VECTOR,
            mass: 1.0,
            is_pinned: false,
            is_selected: false
        }
    }

    pub fn with_mass(position: Vector2, mass: f32) -> Self {
        Particle { 
            position, 
            prev_pos: position,
            init_pos: position,
            velocity: Vector2::UNIT_VECTOR, 
            mass, 
            is_pinned: false,
            is_selected: false
        }
    }

    pub fn at_position(position: Vector2) -> Self {
        Self {
            position,
            prev_pos: position,
            init_pos: position,
            mass: 1.0,
            velocity: Vector2::UNIT_VECTOR,
            is_pinned: false,
            is_selected: false
        }
    }

    pub fn pin(&mut self) {
        self.is_pinned = true;
    }

    // pub fn draw(&self, renderer: &Renderer) {
    //     renderer.draw_point(self.position, SDL_Color {r: , g: todo!(), b: todo!(), a: todo!() });
    // }

    pub fn update(&mut self, delta_time: u32, drag: f32, acceleration: Vector2, elasticity: f32, mouse: &Mouse, window_width: i32, window_height: i32) {
        let mouse_dir = self.position - mouse.pos;
        let mouse_dist = mouse_dir.get_length();
        self.is_selected = mouse_dist < mouse.cursor_size;

        if mouse.left_button_down && self.is_selected {
            let mut difference = mouse.pos - mouse.prev_pos;
            match difference {
                Vector2 {x, y: _} if x > elasticity => difference.x = elasticity,
                Vector2 {x: _, y} if y > elasticity => difference.y = elasticity,
                Vector2 {x, y: _} if x < -elasticity => difference.x = -elasticity,
                Vector2 {x: _, y} if y < -elasticity => difference.y = -elasticity,            
                _ => ()
            };
            self.prev_pos = self.position - difference;
        }

        if mouse.right_button_down && self.is_selected {
            // TODO how to do stick breaking?
        }

        if self.is_pinned {
            self.position = self.init_pos;
            return;
        }

        let new_position = self.position + (self.position - self.prev_pos) * (-1. - drag) + acceleration * (1. - drag) * delta_time * delta_time;

        self.prev_pos = self.position;
        self.position = new_position;

        self.keep_inside_window(window_height, window_width);
    }

    fn keep_inside_window(&mut self, window_height: i32, window_width: i32) {
        // Lock within screen
        if self.position.y > window_height as f32 {
            self.position.y = window_height as f32;
            // particle.prev_pos.y = 0.;
        } else if self.position.y < 0. {
            self.position.y = 0.;
            // particle
        }

        if self.position.x > window_width as f32 {
            self.position.x = window_width as f32;
            // particle.prev_pos.x = 0.;
        } else if self.position.x < 0. {
            self.position.x = 0.;
        }
    }

    // Generalized travel op
    pub fn travel_by_op(mut self, delta_time: f32, acceleration: Vector2, op: fn(Particle, f32, Vector2) -> Vector2) {
        self.prev_pos = self.position;
        self.position = op(self, delta_time, acceleration);
    }

    pub fn verlet_integration(particle: Particle, delta_time: f32, acceleration: Vector2) -> Vector2 {

        let position = Vector2 {
            x: 2. * particle.position.x - particle.prev_pos.x + acceleration.x * (delta_time * delta_time),
            y: 2. * particle.position.y - particle.prev_pos.y + acceleration.y * (delta_time * delta_time),
        };

        position
    }

    pub fn travel_by_verlet(self, delta_time: f32, acceleration: Vector2) {
        self.travel_by_op(delta_time, acceleration, Particle::verlet_integration)
    }

    // Standard travel, dt * velocity
    pub fn travel(mut self, delta_time: f32) {
        self.prev_pos = self.position;
        self.position += self.velocity * delta_time;
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ParticleCollection(HashMap<String, Arc<Particle>>);

impl ParticleCollection {
    pub fn new_empty() -> Self {
        ParticleCollection(HashMap::new())
    }

    pub fn new(particles: HashMap<&str, Particle>) -> Self {
        let mut hm = HashMap::new();
        
        for (key, value) in particles {
            hm.insert(key.to_string(), Arc::new(value));
        }

        ParticleCollection(hm)
    }

    // TODO : Get Mutable reference from particle collection impl `derefmut` trait?
    // pub fn get_particle_mut(&mut self, key: &str) -> &mut Particle {
    //     self.0.get_mut(key).unwrap()
    // }

    pub fn get_particle(&self, key: &str) -> Particle {
        self.0.get(key).unwrap().as_ref().clone()
    }

    pub fn set_particle(&mut self, key: &str, particle: Particle) {
        self.0.insert(key.to_string(), Arc::new(particle));
    }

    pub fn get_position(&self, key: &str) -> Vector2 {
        self.0.get(key).unwrap().position
    }

    pub fn set_position(&mut self, key: &str, new_pos: Vector2) {
        // Get selected Particle
        let mut particle: Particle = self.0.get(key).unwrap().as_ref().clone();
        // Set current position to previous position
        particle.prev_pos = particle.position;
        // Set position to new position
        particle.position = new_pos;
        self.0.insert(key.to_string(), Arc::new(particle));

    }

    pub fn keys(&self) -> Keys<'_, String, Arc<Particle>> {
        self.0.keys()
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}