use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, TextureAccess, WindowCanvas};
use sdl2::EventPump;

use std::time::Duration;

use super::*;
use crate::{APP_NAME, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct NativePlatform {
    canvas: WindowCanvas,
    event_pump: EventPump,
    texture: Texture,
}

impl NativePlatform {
    pub fn new(width: u32, height: u32, scale_factor: u32) -> Box<Self> {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
            .window(APP_NAME, width * scale_factor, height * scale_factor)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let event_pump = context.event_pump().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));

        let texture = canvas
            .texture_creator()
            .create_texture(
                PixelFormatEnum::RGB24,
                TextureAccess::Static,
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .unwrap();

        Box::new(Self {
            canvas,
            event_pump,
            texture,
        })
    }
}

impl Platform for NativePlatform {
    fn init(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }

    fn listen_for_input(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    repeat: false,
                    ..
                } => return true,
                Event::KeyDown { keycode, .. } => {}
                Event::KeyUp { keycode, .. } => {}
                _ => {}
            }
        }
        false
    }

    fn draw_graphics(&mut self, buffer: &[u8]) {
        self.texture.update(None, buffer, SCREEN_WIDTH * 3).unwrap();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present()
    }

    fn sleep(&self, duration: u64) {
        std::thread::sleep(Duration::from_millis(duration));
    }
}
