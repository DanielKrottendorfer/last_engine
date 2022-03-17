use std::time::{Duration, Instant};

use super::settings::MS_PER_UPDATE;



pub struct CatchupTimer{
	start_time: Instant,
	previous_time: Duration,
	lag: Duration,
}

impl CatchupTimer{
	pub fn new() -> Self {
		CatchupTimer{
			start_time: Instant::now(),
			previous_time: Duration::ZERO,
			lag: Duration::ZERO
		}
	}

	pub fn update_lag(&mut self) -> Duration {

		let current = self.start_time.elapsed();
		let elapsed = current - self.previous_time;
		self.previous_time = current;
		self.lag += elapsed;

		self.lag
	}

	pub fn get_iv(&self) -> f32 {
		self.lag.as_secs_f32() / MS_PER_UPDATE.as_secs_f32()
	}

	pub fn update(&mut self) {
		self.lag -= MS_PER_UPDATE;
	}
}