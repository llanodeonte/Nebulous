use crate::Ram;

pub struct Bus {
}

impl Bus {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn read_ram(&self, ram: &Ram, addr: usize) -> u8 {
        ram.read_ram(addr)
    }

    pub fn write_ram(&self, ram: &mut Ram, addr: usize, data: u8) {
        ram.write_ram(addr, data);
    }
}