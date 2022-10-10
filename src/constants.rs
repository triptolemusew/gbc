use std::ops::Range;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

// pub const MEMORY_SIZE: usize = 0x100000;
pub const MEMORY_SIZE: usize = 0x10000;

#[allow(unused)]
pub const WORKING_MEMORY: Range<usize> = 0xA000..0xFE00;

pub const CLOCK_SPEED: i32 = 4194304;
pub const FRAME_RATE: i32 = 60;
