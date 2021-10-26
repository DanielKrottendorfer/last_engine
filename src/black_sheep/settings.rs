use std::time::Duration;

pub const UPS: u64 = 25;
pub const UPS_F32: f32 = UPS as f32;
pub const MS_PER_UPDATE: Duration = Duration::from_millis(1000 / UPS);

pub const INIT_WINDOW_SIZE: (u32, u32) = (800, 800);
