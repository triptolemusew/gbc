mod flag;
mod instructions;
mod opcode;
mod register;

use self::{
    flag::Flags,
    opcode::{ExtendedOpcode, Opcode, EXTENDED_OPCODES, OPCODES},
};
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

    // For debugging
    pub debug: bool,
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

    fn get_byte(&mut self, bus: &Bus) -> u8 {
        let val = bus.read(self.PC as u16);
        self.PC = self.PC.wrapping_add(1);

        val
    }

    fn get_word(&mut self, bus: &Bus) -> u16 {
        let hi = bus.read(self.PC as u16);
        let lo = bus.read(self.PC as u16);
        self.PC = self.PC.wrapping_add(2);
        
        u16::from_be_bytes([lo, hi])
    }

    pub fn step(&mut self, bus: &mut Bus) -> u32 {
        let op = self.get_byte(bus);
        let cycles = match op != 0xCB {
            true => self.execute_opcode(op, bus),
            false => {
                let op = self.get_byte(bus);
                self.execute_extended_opcode(op, bus)
            }
        };

        cycles.clone()
    }

    fn execute_opcode(&mut self, op: u8, bus: &mut Bus) -> &u32 {
        let Opcode(addr, cycles, _) = OPCODES
            .get(op as usize)
            .expect(format!("Standard opcode is not recognized: {:#X}", op).as_str());

        match addr.clone() {
            _ => {}
        }

        cycles
    }

    fn execute_extended_opcode(&mut self, op: u8, bus: &mut Bus) -> &u32 {
        let ExtendedOpcode(addr, cycles, _) = EXTENDED_OPCODES
            .get(op as usize)
            .expect(format!("Extended opcode is not recognized: {:#X}", op).as_str());

        match addr.clone() {
            _ => {}
        }

        cycles
    }
}
