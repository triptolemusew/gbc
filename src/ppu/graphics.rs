use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::{
    pixels::PixelFormatEnum,
    render::{Canvas, Texture, TextureAccess, TextureCreator},
    video::{Window, WindowContext},
};

pub struct Graphics<'a> {
    texture: Texture<'a>,
}

impl<'a> Graphics<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let texture = texture_creator
            .create_texture(
                PixelFormatEnum::RGB24,
                TextureAccess::Static,
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .unwrap();

        Graphics { texture }
    }

    pub fn render(&mut self, buffer: &[u8], canvas: &mut Canvas<Window>) {
        self.texture.update(None, buffer, SCREEN_WIDTH * 3).unwrap();
        canvas.copy(&self.texture, None, None).unwrap();
        canvas.present();
    }
}
