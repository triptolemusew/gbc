use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::rom::Rom;

use crate::ppu::graphics::Graphics;

pub struct Emulator {
    pub cpu: Cpu,
    pub bus: Bus,
}

impl Emulator {
    pub fn new(rom: Rom) -> Self {
        Emulator {
            cpu: Cpu::new(),
            bus: Bus::new(rom),
        }
    }

    pub fn run(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("gbc", 160 * 4, 144 * 4)
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

        let texture_creator = canvas.texture_creator();
        let mut events = sdl_context.event_pump().unwrap();

        let mut graphics = Graphics::new(&texture_creator);

        self.cpu.reset();

        // Default values
        self.bus.write(0xFF05, 0x00);
        self.bus.write(0xFF06, 0x00);
        self.bus.write(0xFF07, 0x00);
        self.bus.write(0xFF10, 0x80);
        self.bus.write(0xFF11, 0xBF);
        self.bus.write(0xFF12, 0xF3);
        self.bus.write(0xFF14, 0xBF);
        self.bus.write(0xFF16, 0x3F);
        self.bus.write(0xFF17, 0x00);
        self.bus.write(0xFF19, 0xBF);
        self.bus.write(0xFF1A, 0x7F);
        self.bus.write(0xFF1B, 0xFF);
        self.bus.write(0xFF1E, 0xBF);
        self.bus.write(0xFF20, 0xFF);
        self.bus.write(0xFF21, 0x00);
        self.bus.write(0xFF22, 0x00);
        self.bus.write(0xFF23, 0xBF);
        self.bus.write(0xFF24, 0x77);
        self.bus.write(0xFF25, 0xF3);
        self.bus.write(0xFF26, 0xF1);
        self.bus.write(0xFF40, 0x91);
        self.bus.write(0xFF42, 0x00);
        self.bus.write(0xFF43, 0x00);
        self.bus.write(0xFF45, 0x00);
        self.bus.write(0xFF47, 0xFC);
        self.bus.write(0xFF48, 0xFF);
        self.bus.write(0xFF49, 0xFF);
        self.bus.write(0xFF4A, 0x00);
        self.bus.write(0xFF4B, 0x00);
        self.bus.write(0xFFFF, 0x00);
    }
}
