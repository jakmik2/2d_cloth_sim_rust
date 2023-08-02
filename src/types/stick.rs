#[derive(Debug, PartialEq)]
pub struct Stick {
    pub p1: String,
    pub p2: String,
    pub length: f32
}

impl Stick {
    pub fn new(p1: &str, p2: &str, length: f32) -> Self {
        Self {
            p1: p1.to_string(),
            p2: p2.to_string(),
            length
        }
    }
}