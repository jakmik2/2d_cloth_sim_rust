use fermium::timer::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency};

use std::io::{stdout, Write};

#[derive(Debug)]
pub struct FrameRateCounter {
    start: f32,
    end: f32,
    sample_amt: i8,
    sample_count: i8,
    current_sample: f32,
}

impl FrameRateCounter {
    pub fn new(sample_count: i8) -> Self {
        Self {
            start: 0.,
            end: 0.,
            sample_amt: 0,
            current_sample: 0.,
            sample_count,
        }
    }

    pub fn start(&mut self) {
        self.start = unsafe {SDL_GetPerformanceCounter()} as f32;
    }

    pub fn update(&mut self) {
        let mut stdout = stdout();
        self.end = unsafe {SDL_GetPerformanceCounter()} as f32;
    
        let elapsed = unsafe { (self.end - self.start) /  SDL_GetPerformanceFrequency() as f32};

        if self.sample_amt < self.sample_count {
            self.current_sample += (1. / elapsed) / self.sample_count as f32;
            self.sample_amt += 1;
        } else {
            print!("\rFPS {}", self.current_sample);
            self.sample_amt = 0;
            self.current_sample = 0.;
        }

        stdout.flush().unwrap()
    }
}