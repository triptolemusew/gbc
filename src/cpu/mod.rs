mod flag;
mod opcode;
mod register;

use self::flag::Flags;
use register::RegisterPair;

use crate::bus::Bus;

#[allow(non_snake_case)]
#[derive(Default)]
pub struct Cpu {
    // Registers
    pub PC: u16,
    pub SP: u16,
    pub AF: RegisterPair,
    pub BC: RegisterPair,
    pub DE: RegisterPair,
    pub HL: RegisterPair,
    pub flags: Flags,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        // self.PC = 0x100;
        self.AF.set(0x01B0);
        self.BC.set(0x0013);
        self.DE.set(0x00D8);
        self.HL.set(0x014D);
        self.SP = 0xFFFE;
    }
}
