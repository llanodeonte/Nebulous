// TODO:
// Set PC and expand functions to allow fetch of first opcode
// Build a function to auto set flag bit based on CpuFlag enum?
// Build out address resolution functions
// Should the PC be incremented at address resolution or during opcode execution?

use crate::bus::Bus;
use crate::ram::Ram;

enum ProgramCounter {
    Next,
    Skip,
    Jump,
}

// May use later for disassembler
// enum Instruction {
//     None, // No Instruction
//     ADC, // Add with Carry
//     AND, // Logical AND
//     ASL, // Arithmetic Shift Left
//     BCC, // Branch if Carry Clear
//     BCS, // Branch if Carry Set
//     BEQ, // Branch if Equal
//     BIT, // Bit Test
//     BMI, // Branch if Minus
//     BNE, // Branch if Not Equal
//     BPL, // Branch if Positive
//     BRK, // Force Interrupt
//     BVC, // Branch if Overflow Clear
//     BVS, // Branch if Overflow Set
//     CLC, // Clear Carry Flag
//     CLD, // Clear Decimal Mode
//     CLI, // Clear Interrupt Disable
//     CLV, // Clear Oveerflow Flag
//     CMP, // Compare
//     CPX, // Compare X Register
//     CPY, // Compare Y Register
//     DEC, // Decrement Memory
//     DEX, // Decrement X Register
//     DEY, // Decrement Y Register
//     EOR, // Exclusive OR
//     INC, // Increment Memory
//     INX, // Increment X Register
//     INY, // Increment Y Register
//     JMP, // Jump
//     JSR, // Jump to Subroutine
//     LDA, // Load Accumulator
//     LDX, // Load X Register
//     LDY, // Load Y Register
//     LSR, // Logical Shift Right
//     NOP, // No Operation
//     ORA, // Logical Inclusive OR
//     PHA, // Push Accumulator
//     PHP, // Push Processor Status
//     PLA, // Pull Accumulator
//     PLP, // Pull Processor Status
//     ROL, // Rotate Left
//     ROR, // Rotate Right
//     RTI, // Return from Interrupt
//     RTS, // Return from Subroutine
//     SBC, // Subtract with Carry
//     SEC, // Set Carry Flag
//     SED, // Set Decimal Flag
//     SEI, // Set Interrupt Disable
//     STA, // Store Accumulator
//     STX, // Store X Register
//     STY, // Store Y Register
//     TAX, // Transfer Accumulator to X
//     TAY, // Transfer Accumulator to Y
//     TSX, // Transfer Stack Pointer to X
//     TXA, // Transfer X to Accumulator
//     TXS, // Transfer X to Stack Pointer
//     TYA, // Transfer Y to Accumulator
// }

#[derive(Debug)]
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

enum CpuFlag {
    C = 1 << 0, // Carry
    Z = 1 << 1, // Zero
    I = 1 << 2, // Interrupt Disable
    D = 1 << 3, // Decimal
    B = 1 << 4, // Break
    U = 1 << 5, // Unused (Always pushed as 1?)
    V = 1 << 6, // Overflow
    N = 1 << 7, // Negative
}

pub struct Cpu {
    a: u8,   // Accumulator
    x: u8,   // X Index
    y: u8,   // Y Index
    pc: u16, // Program Counter
    sp: u8,  // Stack Pointer
    f: u8,   // Status Flags
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            pc: 0x0000,
            sp: 0x00,
            f: 0x00,
        }
    }

    fn set_pc (&mut self, pc_addr: ProgramCounter) {
        self.pc = match pc_addr {
            ProgramCounter::Next => self.pc + 1,
            ProgramCounter::Skip => self.pc + 2,
            ProgramCounter::Jump => panic!("PC Jump not implemented yet."),
        }
    }

    fn fetch_addr(&mut self, addr: AddrMode, bus: &Bus, ram: &Ram) -> u16 {
        match addr {
            // AddrMode::ZPX => self.addr_zpx(), // Zero Page, X
            // AddrMode::ZPY => self.addr_zpy(), // Zero Page, Y
            // AddrMode::ABX => self.addr_abx(), // Absolute, X
            // AddrMode::ABY => self.addr_aby(), // Absolute, Y
            // AddrMode::INX => self.addr_inx(), // Indirect, X
            // AddrMode::INY => self.addr_iny(), // Indirect, Y
            // AddrMode::IMP => self.addr_imp(), // Implicit
            // AddrMode::ACC => self.addr_acc(), // Accumulator
            AddrMode::IMM => self.addr_imm(), // Immediate
            // AddrMode::ZPG => self.addr_zpg(), // Zero Page
            // AddrMode::ABS => self.addr_abs(), // Absolute
            // AddrMode::REL => self.addr_rel(), // Relative
            // AddrMode::IND => self.addr_ind(), // Indirect
            _ => panic!("{:?} is an invalid addressing mode.", addr)
        }
    }

    // 
    fn addr_imm(&mut self) -> u16 {
        let addr = self.pc;
        self.set_pc(ProgramCounter::Next);
        addr
    }

    pub fn cycle(&mut self, bus: &Bus, ram: &Ram) {
        let current_opcode = self.fetch_opcode(bus, ram);
        self.set_pc(ProgramCounter::Next);
        self.execute_opcode(current_opcode, bus, ram);
    }

    // For now, fetches u8 opcode from ram at pc
    fn fetch_opcode(&self, bus: &Bus, ram: &Ram) -> u8 {
        bus.read(ram, self.pc as usize)
    }
    
    // Work in progress opcode mapping/handling
    fn execute_opcode(&mut self, current_opcode: u8, bus: &Bus, ram: &Ram) {
        match current_opcode {
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
            0xA9 => self.opcode_lda(AddrMode::IMM, 2, bus, ram), // LDA #i

            _ => panic!("Unkown opcode {:X?} at PC {:X?}", current_opcode, self.pc),
        }
    }

    fn opcode_lda(&mut self, addr: AddrMode, cycles: u8, bus: &Bus, ram: &Ram) {
        let current_addr = self.fetch_addr(addr, bus, ram);
    }
}