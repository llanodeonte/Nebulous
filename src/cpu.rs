// TODO:
// Set PC and expand functions to allow fetch of first opcode
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
    pub x: u8,   // X Index; Temp pub for testing
    y: u8,   // Y Index
    pc: u16, // Program Counter
    sp: u8,  // Stack Pointer
    p: u8,   // Status Flags

    pub cycles: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            pc: 0x0000,
            sp: 0x00,
            p: 0x00,

            cycles: 0,
        }
    }

    pub fn debug_print(&self) {
        println!(" A: {:02X?}", self.a);
        println!(" X: {:02X?}", self.x);
        println!(" Y: {:02X?}", self.y);
        println!("PC: {:02X?}", self.pc);
        println!("SP: {:02X?}", self.sp);
        println!(" P: {:08b}", self.p);
        // println!("Cycles: {:?}", self.cycles);
    }

    fn set_pc (&mut self, pc_addr: ProgramCounter) {
        self.pc = match pc_addr {
            ProgramCounter::Next => self.pc + 1,
            ProgramCounter::Skip => self.pc + 2,
            ProgramCounter::Jump => panic!("PC Jump not implemented yet."),
        }
    }

    fn set_flag (&mut self, flag: CpuFlag, flag_set: bool) {
        if flag_set {
            self.p |= flag as u8;
        } else {
            self.p &= !(flag as u8);
        }
    }

    fn set_flag_negative_zero (&mut self, value: u8) {
        self.set_flag(CpuFlag::N, (value & 0b1000_0000) != 0);
        self.set_flag(CpuFlag::Z, value == 0);
    }

    fn fetch_addr(&mut self, addr: AddrMode, bus: &Bus, ram: &Ram) -> u16 {
        match addr {
            AddrMode::ZPX => self.addr_zpx(bus, ram), // Zero Page, X
            AddrMode::ZPY => self.addr_zpy(bus, ram), // Zero Page, Y
            // AddrMode::ABX => self.addr_abx(), // Absolute, X
            // AddrMode::ABY => self.addr_aby(), // Absolute, Y
            // AddrMode::INX => self.addr_inx(), // Indirect, X
            // AddrMode::INY => self.addr_iny(), // Indirect, Y
            // AddrMode::IMP => self.addr_imp(), // Implicit
            // AddrMode::ACC => self.addr_acc(), // Accumulator
            AddrMode::IMM => self.addr_imm(), // Immediate
            AddrMode::ZPG => self.addr_zpg(bus, ram), // Zero Page
            // AddrMode::ABS => self.addr_abs(), // Absolute
            // AddrMode::REL => self.addr_rel(), // Relative
            // AddrMode::IND => self.addr_ind(), // Indirect
            _ => panic!("{:?} is an invalid addressing mode.", addr)
        }
    }

    // Sets addr to addr held in current pc
    fn addr_imm(&mut self) -> u16 {
        let addr = self.pc;
        self.set_pc(ProgramCounter::Next);
        println!("Current Addr: {:04X}", addr);
        addr
    }

    // Sets addr hi byte to 00 and lo byte to data at current pc
    fn addr_zpg(&mut self, bus: &Bus, ram: &Ram) -> u16 {
        let addr_lo = bus.read(ram, self.pc as usize);
        self.set_pc(ProgramCounter::Next);
        let addr = (addr_lo as u16) & 0x00FF;
        println!("Current Addr: {:04X}", addr);
        addr
    }

    // Sets addr hi byte to 00 and lo byte to data at current pc + x reg
    fn addr_zpx(&mut self, bus: &Bus, ram: &Ram) -> u16 {
        let addr_lo = bus.read(ram, self.pc as usize).wrapping_add(self.x);
        self.set_pc(ProgramCounter::Next);
        let addr = (addr_lo as u16) & 0x00FF;
        println!("Current Addr: {:04X}", addr);
        addr
    }

    // Sets addr hi byte to 00 and lo byte to data at current pc + y reg
    fn addr_zpy(&mut self, bus: &Bus, ram: &Ram) -> u16 {
        let addr_lo = bus.read(ram, self.pc as usize).wrapping_add(self.y);
        self.set_pc(ProgramCounter::Next);
        let addr = (addr_lo as u16) & 0x00FF;
        println!("Current Addr: {:04X}", addr);
        addr
    }

    pub fn clock(&mut self, bus: &Bus, ram: &Ram) {
        let current_opcode = self.fetch_opcode(bus, ram);
        println!("Current Opcode: {:02X}", current_opcode);
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

            // Load Opcodes
            0xA9 => self.opcode_lda(AddrMode::IMM, 2, bus, ram),
            0xA5 => self.opcode_lda(AddrMode::ZPG, 3, bus, ram),
            0xB5 => self.opcode_lda(AddrMode::ZPX, 4, bus, ram),

            _ => panic!("Unkown opcode {:X?} at PC {:X?}", current_opcode, self.pc),
        }
    }

    fn opcode_lda(&mut self, addr: AddrMode, cycles: u8, bus: &Bus, ram: &Ram) {
        let current_addr = self.fetch_addr(addr, bus, ram);
        self.a = bus.read(ram, current_addr as usize);
        self.set_flag_negative_zero(self.a);
        self.cycles += cycles as usize;
    }
}


// First attempt at using unit testing
// Would like to build more and learn more about it
#[cfg(test)]
mod tests {

    #[test]
    fn set_flag() {
        use crate::*;
        use cpu::*;

        // Test Set Flag Z and N as true and false
        let mut cpu = Cpu::new();
        cpu.set_flag(CpuFlag::Z, true);
        assert_eq!(cpu.p, 0b0000_0010);
        cpu.set_flag(CpuFlag::Z, false);
        assert_eq!(cpu.p, 0b0000_0000);
        cpu.set_flag(CpuFlag::N, true);
        assert_eq!(cpu.p, 0b1000_0000);
        cpu.set_flag(CpuFlag::N, false);
        assert_eq!(cpu.p, 0b0000_0000);
    }

    #[test]
    fn addr_zpy() {
        use crate::*;
        let mut cpu = Cpu::new();
        let bus = Bus::new();
        let mut ram = Ram::new();

        // Test ZPY without wrapping add
        bus.write(&mut ram, 0x0000, 0x2B);
        cpu.y = 0x01;
        let addr = cpu.addr_zpy(&bus, &ram);
        assert_eq!(addr, 0x002C);
        assert_eq!(cpu.pc, 0x0001);

        // Test ZPY with wrapping add
        bus.write(&mut ram, 0x0001, 0xFF);
        cpu.y = 0xA1;
        let addr = cpu.addr_zpy(&bus, &ram);
        assert_eq!(addr, 0x00A0);
        assert_eq!(cpu.pc, 0x0002);

    }

    #[test]
    fn lda() {
        use crate::*;
        use cpu::*;
        let mut cpu = Cpu::new();
        let bus = Bus::new();
        let mut ram = Ram::new();

        // Test LDA with positive non zero data at IMM addr
        bus.write(&mut ram, 0x0000, 0x2B);
        cpu.opcode_lda(AddrMode::IMM, 2, &bus, &ram);
        assert_eq!(cpu.a, 0x2B);
        assert_eq!(cpu.p, 0b0000_0000);
        assert_eq!(cpu.cycles, 2);

        // Test LDA with negative non zero data at IMM addr
        bus.write(&mut ram, 0x0001, 0xA0);
        cpu.opcode_lda(AddrMode::IMM, 4, &bus, &ram);
        assert_eq!(cpu.a, 0xA0);
        assert_eq!(cpu.p, 0b1000_0000);
        assert_eq!(cpu.cycles, 6);

        // Test LDA with non negative zero data at IMM addr
        bus.write(&mut ram, 0x0002, 0x00);
        cpu.opcode_lda(AddrMode::IMM, 4, &bus, &ram);
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.p, 0b0000_0010);
        assert_eq!(cpu.cycles, 10);

        // Test LDA with positive non zero data at ZPG addr
        bus.write(&mut ram, 0x0003, 0x1F);
        bus.write(&mut ram, 0x001F, 0x24);
        cpu.opcode_lda(AddrMode::ZPG, 3, &bus, &ram);
        assert_eq!(cpu.a, 0x24);
        assert_eq!(cpu.p, 0b0000_0000);
        assert_eq!(cpu.cycles, 13);

        // Test LDA with positive non zero data at ZPX addr
        bus.write(&mut ram, 0x0004, 0x2F);
        cpu.x = 0x12;
        bus.write(&mut ram, 0x0041, 0x36);
        cpu.opcode_lda(AddrMode::ZPX, 4, &bus, &ram);
        assert_eq!(cpu.a, 0x36);
        assert_eq!(cpu.p, 0b0000_0000);
        assert_eq!(cpu.cycles, 17);

        // Test LDA with positive non zero data at ZPX addr with wrapping add
        bus.write(&mut ram, 0x0005, 0xFF);
        cpu.x = 0x10;
        bus.write(&mut ram, 0x000F, 0x04);
        cpu.opcode_lda(AddrMode::ZPX, 4, &bus, &ram);
        assert_eq!(cpu.a, 0x04);
        assert_eq!(cpu.p, 0b0000_0000);
        assert_eq!(cpu.cycles, 21);
    }
}