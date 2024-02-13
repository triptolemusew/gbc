use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn new(path: &str) -> Cartridge {
        let mut f = File::open(path).expect("No file found.");
        let metadata = std::fs::metadata(&path).expect("Unable to read file metadata.");

        let mut contents = vec![0; metadata.len() as usize];

        f.read_exact(&mut contents).expect("Buffer overflow");

        Cartridge { data: contents }
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}
