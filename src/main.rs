extern crate sdl2;

#[macro_use]
extern crate lazy_static;

use rom::Rom;
use std::env;

mod cpu;
mod rom;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).unwrap();

    let rom = Rom::new(&file_name);
}
