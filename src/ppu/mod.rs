use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

const CLOCK_SPEED: u32 = 4194304;
const FRAME_RATE: u32 = 60;
const CYCLES_PER_FRAME: u32 = CLOCK_SPEED / FRAME_RATE;

const LCD_CONTROL_ADDR: u16 = 0xFF40;
const LCD_STATUS_ADDR: u16 = 0xFF41;

const CURRENT_SCANLINE_ADDR: u16 = 0xFF44;

const OAM_PERIOD: u32 = 80;
const TRANSFER_PERIOD: u32 = OAM_PERIOD + 172;
const HBLANK_PERIOD: u32 = 456;
const FRAME_PERIOD: u32 = HBLANK_PERIOD + 144;
const VBLANK_PERIOD: u32 = FRAME_PERIOD + 4560;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LcdMode {
    HBlank = 0,
    VBlank = 1,
    SpriteSearch = 2,
    LcdTransfer = 3,
}

enum StatusInterrupt {
    HBlank = 0b00001000,
    VBlank = 0b00010000,
    Oam = 0b00100000,
    Coincidence = 0b01000000,
}

/*
    PPU has 4 distinct modes:
    Mode 0: HBlank
    Mode 1: VBlank
    Mode 2: OAM Scan
    Mode 3: HDraw

    Modes 0, 2 and 3 doesn't occur during VBlank, only during VDraw.
    Mode transitions occur in this order.

    1 -> 2 VBlank to VDraw transition
    2 -> 3 VDraw only
    3 -> 0 VDraw only
    0 -> 1 VDraw to VBlank transition
    0 -> 2 VDraw only

    One frame takes 70224 cycles (~59.7275 frames per second).

    Timing is divided into 154 lines.
    144 during VDraw.
    10 during VBlank.

    Each line takes 456 cycles.
*/

pub fn is_bit_set(data: &u8, position: usize) -> bool {
    (data & (1 << position)) > 0
}

pub fn get_bit_val(data: &u8, position: u8) -> u8 {
    match (data & (1 << position)) > 0 {
        true => 1,
        false => 0,
    }
}

// TODO: Rename this enum
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum SdlColor {
    White = 0xEE,     // 0 White
    LightGrey = 0x99, // 1 Light Gray
    DarkGrey = 0x66,  // 2 Dark Gray
    Black = 0x22,     // 3 Black
}

#[derive(Default)]
pub struct Ppu {
    pub scanline_cycles: u32,
    pub frame_cycles: u32,
    pub frame_buffer: Vec<u8>,
    pub vram: Vec<u8>,
    pub oam: Vec<u8>,
    pub ly: u8,
    pub lyc: u8,
    pub lcdc: u8,
    pub stat: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub scy: u8,
    pub scx: u8,
    pub wy: u8,
    pub wx: u8,
}

#[allow(clippy::new_without_default)]
impl Ppu {
    pub fn new() -> Self {
        Ppu {
            frame_buffer: vec![0xff; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
            vram: vec![0; 0xFFFF],
            oam: vec![0; 0xFFFF],
            ..Default::default()
        }
    }

    pub fn step(&mut self, cycles: u32) {}
}
