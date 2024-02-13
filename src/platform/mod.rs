#[cfg(feature = "sdl")]
mod native;

#[cfg(feature = "sdl")]
pub use native::NativePlatform;

pub trait Platform {
    fn init(&mut self);
    fn listen_for_input(&mut self) -> bool;
    fn draw_graphics(&mut self, buffer: &[u8]);
    fn sleep(&self, duration: u64);
}
