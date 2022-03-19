use std::time::{Duration, Instant};

use super::settings::MS_PER_UPDATE;

pub struct CatchupTimer {
    start_time: Instant,
    previous_update_time: Duration,
    lag: Duration,

    previous_fps_time: Duration,
    fps: u32,
}

impl CatchupTimer {
    pub fn new() -> Self {
        CatchupTimer {
            start_time: Instant::now(),
            previous_update_time: Duration::ZERO,
            lag: Duration::ZERO,

            previous_fps_time: Duration::ZERO,
            fps: 0,
        }
    }

    pub fn should_update(&mut self) -> bool {
        let current = self.start_time.elapsed();
        let elapsed = current - self.previous_update_time;
        self.previous_update_time = current;
        self.lag += elapsed;

        if self.lag >= MS_PER_UPDATE {
            self.lag -= MS_PER_UPDATE;
            true
        } else {
            self.fps += 1;
            if current - self.previous_fps_time > Duration::from_secs(1) {
                println!("fps: {}", self.fps);
                self.previous_fps_time = current;
                self.fps = 0;
            }
            false
        }
    }

    pub fn get_iv(&self) -> f32 {
        self.lag.as_secs_f32() / MS_PER_UPDATE.as_secs_f32()
    }
}
