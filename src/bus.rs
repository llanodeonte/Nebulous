// Add memory map guards based on addressing

// $0000–$07FF 	$0800 	2 KB internal RAM
// $0800–$0FFF 	$0800 	
// $1000–$17FF 	$0800   Mirrors of $0000–$07FF
// $1800–$1FFF 	$0800
// $2000–$2007 	$0008 	NES PPU registers
// $2008–$3FFF 	$1FF8 	Mirrors of $2000–$2007 (repeats every 8 bytes)
// $4000–$4017 	$0018 	NES APU and I/O registers
// $4018–$401F 	$0008 	APU and I/O functionality that is normally disabled. See CPU Test Mode.
// $4020–$FFFF 	$BFE0 	Cartridge space: PRG ROM, PRG RAM, and mapper registers

use crate::ram::Ram;

pub struct Bus {
}

impl Bus {
    pub fn new() -> Self {
        Self {
        }
    }

    // Update naming conventions to reflect broader addressing
    pub fn read(&self, ram: &Ram, addr: usize) -> u8 {
        match addr {
            // Finish building full address range with temp panics for each function
            0x0000..=0x1FFF => ram.read(addr & 0x7FF),
            _ => panic!("Address {:?} outside valid read range", addr)
        }
    }

    pub fn write(&self, ram: &mut Ram, addr: usize, data: u8) {
        match addr {
            0x0000..=0x1FFF => ram.write(addr & 0x7FF, data),
            _ => panic!("Address {:?} outside valid write range", addr)
        }
    }
}