use std::convert::From;
use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct RegisterPair {
    pub hi: u8,
    pub lo: u8,
}

impl RegisterPair {
    pub fn set(&mut self, word: u16) {
        // Big endian.  e.g. 0x1234, hi -> 0x12, lo -> 0x34
        let [hi, lo] = word.to_be_bytes();

        self.hi = hi as u8;
        self.lo = lo as u8;
    }

    fn increment(&mut self) {
        let val = u16::from(RegisterPair {
            hi: self.hi,
            lo: self.lo,
        });
        let (value, _) = val.overflowing_add(1);
        let result = RegisterPair::from(value);

        self.lo = result.lo;
        self.hi = result.hi;
    }

    fn decrement(&mut self) {
        let val = u16::from(RegisterPair {
            hi: self.hi,
            lo: self.lo,
        });
        let (value, _) = val.overflowing_sub(1);
        let result = RegisterPair::from(value);

        self.lo = result.lo;
        self.hi = result.hi;
    }

    fn add(&mut self, val: RegisterPair) {
        let own = u16::from(RegisterPair {
            hi: self.hi,
            lo: self.lo,
        });
        let rhs = u16::from(RegisterPair {
            hi: val.hi,
            lo: val.lo,
        });
        let result = own.wrapping_add(rhs);
        self.set(result);
    }
}

impl Default for RegisterPair {
    fn default() -> Self {
        Self { hi: 0, lo: 0 }
    }
}

impl Debug for RegisterPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#X}", u16::from_be_bytes([self.hi, self.lo]))
    }
}

impl From<u16> for RegisterPair {
    fn from(word: u16) -> Self {
        let hi = (word >> 8) as u8;
        let lo = (word & 0xFF) as u8;
        Self { hi, lo }
    }
}

impl From<RegisterPair> for u16 {
    fn from(reg: RegisterPair) -> u16 {
        u16::from_be_bytes([reg.hi, reg.lo])
    }
}
