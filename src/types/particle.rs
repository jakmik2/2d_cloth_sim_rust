use std::{sync::Arc, collections::{HashMap, hash_map::Keys}};

use crate::types::vector2::Vector2;

// A basic particle containing Position, Velocity and Mass
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Particle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub mass: f32,
    pub prev_pos: Vector2
}

impl Particle {
    pub fn new() -> Self {
        Particle { 
            position: Vector2::UNIT_VECTOR,
            velocity: Vector2::UNIT_VECTOR,
            mass: 1.0,
            prev_pos: Vector2::UNIT_VECTOR
        }
    }

    pub fn with_mass(position: Vector2, mass: f32) -> Self {
        Particle { 
            position, 
            velocity: Vector2::UNIT_VECTOR, 
            mass, 
            prev_pos: position
        }
    }

    pub fn at_position(position: Vector2) -> Self {
        Self {
            position,
            prev_pos: position,
            mass: 1.0,
            velocity: Vector2::UNIT_VECTOR
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

        println!("{:?}", position);

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

#[derive(Clone)]
pub struct ParticleCollection(HashMap<String, Arc<Particle>>);

impl ParticleCollection {
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