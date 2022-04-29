use std::time::Duration;

pub const UPS: u64 = 25;
pub const UPS_F32: f32 = UPS as f32;
pub const MS_PER_UPDATE: Duration = Duration::from_millis(1000 / UPS);

pub const INIT_WINDOW_SIZE: [u32; 2] = [1800, 900];
pub const INIT_WINDOW_SIZE_I32: [i32; 2] = [INIT_WINDOW_SIZE[0] as i32, INIT_WINDOW_SIZE[1] as i32];
pub const INIT_WINDOW_SIZE_F32: [f32; 2] = [INIT_WINDOW_SIZE[0] as f32, INIT_WINDOW_SIZE[1] as f32];
