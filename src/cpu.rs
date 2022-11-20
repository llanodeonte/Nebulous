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
}