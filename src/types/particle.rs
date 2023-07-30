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