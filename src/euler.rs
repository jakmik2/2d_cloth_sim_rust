use crate::types::{particle::*, vector2::*};

pub fn euler_main() {
    let mut particle = Particle::default();

    let force = 10.0;
    let acceleration = (force * Vector2::UNIT_VECTOR) / particle.mass;

    let mut time = 0.0;
    let delta_time = 0.00001;

    while time <= 10.0 {
        println!("time = {:?}    \tposition = {:?}     \tvelocity = {:?}", time, particle.position, particle.velocity);
        particle.velocity += acceleration * delta_time;
        particle.position += particle.velocity * delta_time;

        time += delta_time;
    }
}