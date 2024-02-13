use crate::cartridge::Cartridge;
use crate::ppu::Ppu;

pub struct Bus {
    ppu: Ppu,
    cart: Cartridge,
    wram: [u8; 0x2000],
    hram: [u8; 0x2000],
}

impl Bus {
    pub fn new(cart: Cartridge) -> Self {
        Bus {
            cart,
            ppu: Ppu::new(),
            wram: [0; 0x2000],
            hram: [0; 0x2000],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        1
    }

    pub fn write(&mut self, addr: u16, data: u8) {}

    pub fn step(&mut self, cycles: usize) {
        self.ppu.step(cycles as u32);
    }
}
