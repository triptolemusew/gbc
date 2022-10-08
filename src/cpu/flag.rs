use std::convert::From;
use std::fmt::Debug;

const Z_FLAG_POSITION: u8 = 7;
const N_FLAG_POSITION: u8 = 6;
const H_FLAG_POSITION: u8 = 5;
const C_FLAG_POSITION: u8 = 4;

#[derive(Clone, Copy)]
#[allow(non_snake_case)]
pub struct Flags {
    pub Z: bool,
    pub N: bool,
    pub H: bool,
    pub C: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            Z: false,
            N: false,
            H: false,
            C: false,
        }
    }
}

impl Flags {
    pub fn clear(&mut self) {
        *self = Default::default();
    }
}

impl From<Flags> for u8 {
    fn from(flag: Flags) -> u8 {
        (if flag.Z { 1 } else { 0 }) << Z_FLAG_POSITION
            | (if flag.N { 1 } else { 0 }) << N_FLAG_POSITION
            | (if flag.H { 1 } else { 0 }) << H_FLAG_POSITION
            | (if flag.C { 1 } else { 0 }) << C_FLAG_POSITION
    }
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Flags {
            Z: ((byte >> Z_FLAG_POSITION) & 0b1) != 0,
            N: ((byte >> N_FLAG_POSITION) & 0b1) != 0,
            H: ((byte >> H_FLAG_POSITION) & 0b1) != 0,
            C: ((byte >> C_FLAG_POSITION) & 0b1) != 0,
        }
    }
}

impl Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Z: {}, N: {}, H: {}, C: {}",
            self.Z, self.N, self.H, self.C
        )
    }
}
