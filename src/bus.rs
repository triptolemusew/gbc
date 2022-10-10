use crate::ppu::Ppu;
use crate::rom::Rom;

pub struct Bus {
    ppu: Ppu,
    rom: Rom,
    wram: [u8; 0x2000],
    hram: [u8; 0x2000],
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Bus {
            rom,
            ppu: Ppu::new(),
            wram: [0; 0x2000],
            hram: [0; 0x2000],
        }
    }

    pub fn step(&mut self, cycles: usize) {
        self.ppu.step(cycles as u32);
    }
}
