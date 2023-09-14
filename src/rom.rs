// Build in error handling for ROM loading
// Will it be neccessary to load a BIOS of some sort?

use std::env;
use std::fs;

pub struct Rom {
    pub buffer: Vec<u8>
}

impl Rom {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    pub fn load_rom(&mut self) {
        let rom_file_path = env::args().nth(1).unwrap();
        self.buffer = fs::read(rom_file_path).unwrap();
    }
}