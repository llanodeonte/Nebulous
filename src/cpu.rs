use crate::{Bus, Ram};

enum AddrMode {
    ZPX, // Zero Page Indexed X
    ZPY, // Zero Page Indexed Y
    ABX, // Absolute Indexed X
    ABY, // Absolute Indexed Y
    INX, // Indirect Indexed X
    INY, // Indirect Indexed Y
    IMP, // Implicit
    ACC, // Accumulator
    IMM, // Immediate
    ZPG, // ZeroPage
    ABS, // Absolute
    REL, // Relative
    IND, // Indirect
}

pub struct Cpu {
    a: u8,   // Accumulator
    x: u8,   // X Index
    y: u8,   // Y Index
    pc: u16, // Program Counter
    s: u8,   // Stack Pointer
    p: u8,   // Status Register
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0,
            p: 0,
        }
    }

    // For now, fetches u8 opcode from ram at pc
    pub fn fetch_opcode(&self, bus: &Bus, ram: &Ram) -> u8 {
        bus.read_ram(ram, self.pc as usize)
    }
}