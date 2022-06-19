use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Rom {
    pub data: Vec<u8>,
}

impl Rom {
    pub fn new(path: &str) -> Rom {
        let mut f = File::open(path).expect("No file found.");
        let metadata = std::fs::metadata(&path).expect("Unable to read file metadata.");

        let mut contents = vec![0; metadata.len() as usize];

        f.read_exact(&mut contents).expect("Buffer overflow");

        Rom { data: contents }
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }
}
