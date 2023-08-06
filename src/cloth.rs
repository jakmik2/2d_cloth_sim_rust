use crate::{mouse::Mouse, types::{vector2::Vector2, particle::*, stick::Stick}, renderer::Renderer};

#[derive(Debug)]
pub struct Cloth {
    gravity: Vector2,
    drag: f32,
    elasticity: f32,
    points: ParticleCollection,
    sticks: Vec<Stick>,
    width: i32,
    height: i32
}

impl Cloth {
    pub fn new() -> Self {        
        Self {
            gravity: Vector2 {x: 0., y: 981.},
            drag: 0.01,
            elasticity: 10.,
            points: ParticleCollection::new_empty(),
            sticks: Vec::new(),
            width: 0,
            height: 0
        }
    }

    pub fn init(&mut self, width: i32, height: i32, spacing: i32, start_x: i32, start_y: i32) {   
        self.width = width;
        self.height = height;
        
        for y in 0..height {
            for x in 0..width {
                let mut point = Particle::at_position(Vector2 {x: (start_x + x * spacing) as f32, y: (start_y + y * spacing) as f32});

                if x != 0 {
                    let point_idx = format!("{},{}", x - 1, y);
                    let left_point = self.points.get_particle(point_idx.as_str());
                    let stick = Stick::new(
                        format!("{},{}", x, y).as_str(),
                        format!("{},{}", x - 1, y).as_str(),
                        point.position.get_distance_to(left_point.position)
                    );
                    self.sticks.push(stick);
                }

                if y != 0 {
                    let point_idx = format!("{},{}", x, y - 1);
                    let up_point = self.points.get_particle(point_idx.as_str());
                    let stick = Stick::new(
                        format!("{},{}", x, y).as_str(),
                        format!("{},{}", x, y - 1).as_str(),
                        point.position.get_distance_to(up_point.position)
                    );
                    self.sticks.push(stick);
                }

                if y == 0 && x < self.width {
                    point.pin();
                }

                self.points.set_particle(format!("{},{}", x, y).as_str(), point);
            }
        }
    }
        
    pub fn update(&mut self, renderer: &Renderer, mouse: &Mouse, delta_time: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let key = format!("{},{}", x, y);

                let mut point = self.points.get_particle(key.as_str());
                point.update(delta_time, self.drag, self.gravity, self.elasticity, mouse, renderer.window_width, renderer.window_height);
                
                self.points.set_particle(key.as_str(), point);
            }
        }

        for idx in 0..self.sticks.len() {
            self.sticks[idx].update(&mut self.points);
        }
    }

    pub fn draw(&self, renderer: &Renderer) {
        for idx in 0..self.sticks.len() {
            self.sticks[idx].draw(renderer, &self.points);
        }
    }
}