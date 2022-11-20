pub struct Ram {
    ram: [u8; 0xFFFF],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            ram: [0; 0xFFFF],
        }
    }

    pub fn read_ram(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    pub fn write_ram(&mut self, addr: usize, data: u8) {
        self.ram[addr] = data;
    }
}