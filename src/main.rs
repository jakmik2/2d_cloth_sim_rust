pub mod types;
pub mod euler;

use macroquad::prelude::*;
use types::{particle::Particle, vector2::Vector2, stick::Stick};


#[macroquad::main("2d Cloth Simulation")]
async fn main() {
    // Make Particles
    let p_a = Particle::with_mass(Vector2 {x: 220., y: 20.}, 1.);
    let p_b = Particle::with_mass(Vector2 {x: 280., y: 20.}, 1.);
    // let p_c = Particle::with_mass(Vector2 {x: 220., y: 60.}, 1.);
    // let p_d = Particle::with_mass(Vector2 {x: 280., y: 60.}, 1.);
    

    // Add four Particles
    let mut particles: Vec<Particle> = vec![
        p_a,
        p_b,
        // p_c,
        // p_d
    ];

    // Add four sticks
    let mut sticks: Vec<Stick> = vec![
        Stick::new(p_a, p_b),
        // Stick::new(p_b, p_c),
        // Stick::new(p_c, p_d),
        // Stick::new(p_d, p_a),
    ];
    
    draw(&particles);

    loop {
        // Verlet Integration
        clear_background(LIGHTGRAY);

        update(&mut particles, &mut sticks);

        draw(&particles);

        next_frame().await
    }
}


fn draw(particles: &Vec<Particle>) {
    for particle in particles {
        draw_circle(particle.position.x, particle.position.y, 10., YELLOW)
    }
}


fn update(particles: &mut Vec<Particle>, sticks: &mut Vec<Stick>) {
    let delta_time = get_frame_time();

    for particle in particles {
        let force: Vector2 = Vector2 {x: 0., y: 2000.};
        
        let acceleration = Vector2 {x: force.x / particle.mass, y: force.y / particle.mass};

        let new_prev_position = particle.position.clone();

        particle.position = Vector2 {
            x: 2. * particle.position.x - particle.prev_pos.x + acceleration.x * (delta_time * delta_time),
            y: 2. * particle.position.y - particle.prev_pos.y + acceleration.y * (delta_time * delta_time),
        };

        particle.prev_pos = new_prev_position;

        // Lock within screen
        if particle.position.y > screen_height() {
            particle.position.y = screen_height();
            // particle.prev_pos.y = 0.;
        } else if particle.position.y < 0. {
            particle.position.y = 0.;
            // particle
        }

        if particle.position.x > screen_width() {
            particle.position.x = screen_width();
            // particle.prev_pos.x = 0.;
        } else if particle.position.x < 0. {
            particle.position.x = 0.;
        }

        // particle.travel_by_verlet(delta_time, acceleration);
        println!("{:?} PARTICLE", particle.position)
    }

    for stick in sticks {
        println!("{:?} STICK", stick.p1.position);
        println!("--------------");
    }
}