pub mod types;
pub mod application;
pub mod mouse;
pub mod cloth;
pub mod renderer;

use fermium::SDL_Quit;
// use macroquad::prelude::*;
use application::*;

fn main() {
    let mut app = Application::setup(1200,320, 20);
    while app.is_running() {
        app.input();
        app.update();
        app.render();
    }
    unsafe { SDL_Quit() };
}

// #[macroquad::main("2d Cloth Simulation")]
// async fn main() {
//     // Make Particles
//     let p_a = Particle::with_mass(Vector2 {x: 220., y: 20.}, 1.);
//     let p_b = Particle::with_mass(Vector2 {x: 280., y: 20.}, 1.);
//     let p_c = Particle::with_mass(Vector2 {x: 280., y: 60.}, 1.);
//     let p_d = Particle::with_mass(Vector2 {x: 220., y: 60.}, 1.);
    
//     // Make Map
//     let mut particle_map = HashMap::new();

//     particle_map.insert("p_a", p_a);
//     particle_map.insert("p_b", p_b);
//     particle_map.insert("p_c", p_c);
//     particle_map.insert("p_d", p_d);

//     // Add Particles
//     let mut particles: ParticleCollection = ParticleCollection::new(particle_map);

//     // Add four sticks
//     let sticks: Vec<Stick> = vec![
//         // Stick::new("p_a", "p_b", p_a.position.get_distance_to(p_b.position)),
//         Stick::new("p_b", "p_c", p_b.position.get_distance_to(p_c.position)),
//         // Stick::new("p_c", "p_d", p_c.position.get_distance_to(p_d.position)),
//         Stick::new("p_d", "p_a", p_d.position.get_distance_to(p_a.position)),
//         // Stick::new("p_a", "p_c", p_a.position.get_distance_to(p_c.position)),
//         Stick::new("p_b", "p_d", p_b.position.get_distance_to(p_d.position)),
//     ];
    
//     draw(&particles, &sticks);

//     loop {
//         // Verlet Integration
//         clear_background(LIGHTGRAY);

//         update(&mut particles, &sticks);

//         draw(&particles, &sticks);

//         next_frame().await
//     }
// }


// fn draw(particles: &ParticleCollection, sticks: &Vec<Stick>) {
//     for key in particles.keys() {
//         let particle_pos = particles.get_position(key);
//         draw_circle(particle_pos.x, particle_pos.y, 10., BLACK)
//     }

//     for stick in sticks {
//         let p1 = particles.get_position(stick.p1.as_str());
//         let p2 = particles.get_position(stick.p2.as_str());
        
//         draw_line(p1.x, p1.y, p2.x, p2.y, 1., WHITE)
//     }
// }


// fn update(particles: &mut ParticleCollection, sticks: &Vec<Stick>) {
//     let delta_time = get_frame_time();

//     for key in particles.clone().keys() {
//         let mut particle = particles.get_particle(&key);

//         let force: Vector2 = Vector2 {x: 0., y: 2000.};
        
//         let acceleration = Vector2 {x: force.x / particle.mass, y: force.y / particle.mass};

//         let new_prev_position = particle.position.clone();

//         particle.position = Vector2 {
//             x: 2. * particle.position.x - particle.prev_pos.x + acceleration.x * (delta_time * delta_time),
//             y: 2. * particle.position.y - particle.prev_pos.y + acceleration.y * (delta_time * delta_time),
//         };

//         particle.prev_pos = new_prev_position;

//         // Lock within screen
//         if particle.position.y > screen_height() {
//             particle.position.y = screen_height();
//             // particle.prev_pos.y = 0.;
//         } else if particle.position.y < 0. {
//             particle.position.y = 0.;
//             // particle
//         }

//         if particle.position.x > screen_width() {
//             particle.position.x = screen_width();
//             // particle.prev_pos.x = 0.;
//         } else if particle.position.x < 0. {
//             particle.position.x = 0.;
//         }
        
//         particles.set_particle(key, particle);
//     }

//     for stick in sticks {
//         let mut p1 = particles.get_particle(stick.p1.as_str());
//         let mut p2 = particles.get_particle(stick.p2.as_str());
        
//         let diff = p1.position - p2.position;
//         let diff_factor = (stick.length - diff.get_length()) / diff.get_length() * 0.5;
//         let offset: Vector2 = diff * diff_factor;

//         p1.position += offset;
//         p2.position -= offset;

//         particles.set_particle(stick.p1.as_str(), p1);
//         particles.set_particle(stick.p2.as_str(), p2);
//     }
// }