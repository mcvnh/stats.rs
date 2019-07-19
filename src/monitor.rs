// use std::time::{Duration, SystemTime, UNIX_EPOCH};
use web_sys::{Performance, Window};

// use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct CanvasPerformance {
    pub begin_time: f64,
    pub previous_time: f64,
    pub frames: u32,
    performance: Performance,
    pub fps: f64,
    pub ms: f64,
}

impl CanvasPerformance {
    pub fn init(window: &Window) -> Self {
        let performance = window
            .performance()
            .expect("performance should be available");

        CanvasPerformance {
            begin_time: performance.now(),
            previous_time: performance.now(),
            performance,
            frames: 0,
            fps: 0.0,
            ms: 0.0,
        }
    }

    pub fn recalculate(&mut self) {
        self.frames += 1;

        let latest_time = self.performance.now();

        self.ms = ((latest_time - self.begin_time) as u32) as f64;

        if latest_time >= (self.previous_time + 1_000.0) {
            let fps = (self.frames * 1_000) as f64 / (latest_time - self.previous_time);
            self.fps = fps as u32 as f64;

            self.previous_time = latest_time;
            self.frames = 0;
        }

        self.begin_time = latest_time;
    }

    pub fn set_begin_time(&mut self, time: f64) {
        self.begin_time = time;
    }

    pub fn set_previous_time(&mut self, time: f64) {
        self.previous_time = time;
    }
}
