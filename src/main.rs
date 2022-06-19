use std::env;

use rom::Rom;

mod rom;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).unwrap();

    let rom = Rom::new(&file_name);
}
