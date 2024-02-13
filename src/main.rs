use gbc::{cartridge::Cartridge, emulator::Emulator};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).unwrap();

    let rom = Cartridge::new(&file_name);

    let mut emulator = Emulator::new(rom);

    emulator.run();
}
