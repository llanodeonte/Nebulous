pub struct Ram {
    ram: [u8; 0xFFFF],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            ram: [0; 0xFFFF],
        }
    }

    pub fn read(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    pub fn write(&mut self, addr: usize, data: u8) {
        self.ram[addr] = data;
    }
}