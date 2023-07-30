use crate::types::{vector2::*, particle::*};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Stick {
    pub p1: Particle,
    pub p2: Particle,
    pub length: f32
}

impl Stick {
    pub fn new(p1: Particle, p2: Particle) -> Self {
        Self {
            p1,
            p2,
            length: p1.position.get_distance_to(p2.position)
        }
    }
}