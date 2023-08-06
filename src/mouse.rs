use crate::types::vector2::Vector2;

#[derive(Debug)]
pub struct Mouse {
    pub pos: Vector2,
    pub prev_pos: Vector2,
    pub cursor_size: f32,
    max_cursor_size: f32,
    min_cursor_size: f32,
    pub left_button_down: bool,
    pub right_button_down: bool
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            pos: Vector2::ZERO,
            prev_pos: Vector2::ZERO,
            cursor_size: 20.,
            max_cursor_size: 100.,
            min_cursor_size: 20.,
            left_button_down: false,
            right_button_down: false,
        }
    }


    pub fn increase_cursor_size(&mut self, increment: f32) {
        if self.cursor_size + increment > self.max_cursor_size || self.cursor_size + increment < self.min_cursor_size {
            return;
        }
        self.cursor_size += increment;
    }

    pub fn update_position(&mut self, x: i32, y: i32) {
        self.prev_pos = self.pos.clone();
        self.pos = Vector2 {x: x as f32, y: y as f32}
    }
}