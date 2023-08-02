use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign, SubAssign};

// Position in 2d Space
#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

// Add Vectors
impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Subtract Vectors
impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

// Multiply Vector By f32
impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: rhs.x * self,
            y: rhs.y * self
        }
    }
}

// Multiply Vector2 by Vector2
impl Mul for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

// Divide Vector2
impl Div<f32> for Vector2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl Vector2 {
    // Defualt values
    pub const UNIT_VECTOR: Vector2 = Vector2 {x: 1.0, y: 1.0};

    pub fn get_length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn get_distance_to(self, p2: Vector2) -> f32 {
        (self - p2).get_length()
    }
}