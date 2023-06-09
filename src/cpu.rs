use crate::bus::Bus;
use crate::ram::Ram;

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

    // Work in progress opcode mapping/handling
    fn execute_opcode(&self) {
        match opcode {
            // Work in progress opcode table

            // Control Opcodes
            // 0x00 => self.brk(), // BRK
            // 0x04 => self.nop(), // NOP d
            // 0x08 => self.php(), // PHP
            // 0x0C => self.nop(), // NOP a
            // 0x10 => self.bpl(), // BPL *+d
            // 0x14 => self.nop(), // NOP d,x
            // 0x18 => self.clc(), // CLC
            // 0x1C => self.nop(), // NOP a,x
            // 0x20 => self.jsr(), // JSR a
            // 0x24 => self.bit(), // BIT d
            // 0x28 => self.plp(), // PLP
            // 0x2C => self.bit(), // BIT a
            // 0x30 => self.bmi(), // BMI *+d
            // 0x34 => self.nop(), // NOP d,x
            // 0x38 => self.sec(), // SEC
            // 0x3C => self.nop(), // NOP a,x
            // 0x40 => self.rti(), // RTI
            // 0x44 => self.nop(), // NOP d
            // 0x48 => self.pha(), // PHA
            // 0x4C => self.jmp(), // JMP a
            // 0x50 => self.bvc(), // BVC *+d
            // 0x54 => self.nop(), // NOP d,x
            // 0x58 => self.cli(), // CLI
            // 0x5C => self.nop(), // NOP a,x
            // 0x60 => self.rts(), // RTS
            // 0x64 => self.nop(), // NOP d
            // 0x68 => self.pla(), // PLA
            // 0x6C => self.jmp(), // JMP (a)
            // 0x70 => self.bvs(), // BVS *+d
            // 0x74 => self.nop(), // NOP d,x
            // 0x78 => self.sei(), // SEI
            // 0x7C => self.nop(), // NOP a,x
            // 0x80 => self.nop(), // NOP #i
            // 0x84 => self.sty(), // STY d
            // 0x88 => self.dey(), // DEY
            // 0x8C => self.sty(), // STY a
            // 0x90 => self.bcc(), // BCC *+d
            // 0x94 => self.sty(), // STY d,x
            // 0x98 => self.tya(), // TYA
            // 0x9C => self.shy(), // SHY a,x
            // 0xA0 => self.ldy(), // LDY #i
            // 0xA4 => self.ldy(), // LDY d
            // 0xA8 => self.tay(), // TAY
            // 0xAC => self.ldy(), // LDY a
            // 0xB0 => self.bcs(), // BCS *+d
            // 0xB4 => self.ldy(), // LDY d,x
            // 0xB8 => self.clv(), // CLV
            // 0xBC => self.ldy(), // LDY a,x
            // 0xC0 => self.cpy(), // CPY #i
            // 0xC4 => self.cpy(), // CPY d
            // 0xC8 => self.iny(), // INY
            // 0xCC => self.cpy(), // CPY a
            // 0xD0 => self.bne(), // BNE *+d
            // 0xD4 => self.nop(), // NOP d,x
            // 0xD8 => self.cld(), // CLD
            // 0xDC => self.nop(), // NOP a,x
            // 0xE0 => self.cpx(), // CPX #i
            // 0xE4 => self.cpx(), // CPX d
            // 0xE8 => self.inx(), // INX
            // 0xEC => self.cpx(), // CPX a
            // 0xF0 => self.beq(), // BEQ *+d
            // 0xF4 => self.nop(), // NOP d,x
            // 0xF8 => self.sed(), // SED
            // 0xFC => self.nop(), // NOP a,x

            // ALU Opcodes

            _ => panic!("Unkown opcode {:X?} at PC {:X?}", current_opcode, self.pc),
        }
    }
}