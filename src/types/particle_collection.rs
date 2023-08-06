use std::collections::HashMap;

use super::particle::Particle;

#[derive(Debug)]
pub struct ParticleCollection {
    particles: HashMap<String, Particle>,
}

impl<'a> ParticleCollection {
    pub fn new(particles: HashMap<String, Particle>) -> Self {
        Self {
            particles
        }
    }

    pub fn get_ref(self, particle: String) -> &'a mut Particle {
        &mut self.particles.get(&particle).unwrap()
    }
}