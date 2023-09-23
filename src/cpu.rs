// TODO:
// Set PC (0xFFFC?)
//     and expand functions to allow fetch of first opcode from live ROM
// Build out address resolution functions

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
    pub y: u8,   // Y Index; Temp pub for testing
    pc: u16, // Program Counter
    sp: u8,  // Stack Pointer
    p: u8,   // Status Flags

    pub cycles: usize,
    page_crossed: bool,
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
            page_crossed: false,
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
        self.page_crossed = false;
        match addr {
            // AddrMode::IMP => self.addr_imp(), // Implicit
            // AddrMode::ACC => self.addr_acc(), // Accumulator
            AddrMode::IMM => self.addr_imm(), // Immediate
            AddrMode::ZPG => self.addr_zpg(bus, ram), // Zero Page
            AddrMode::ZPX => self.addr_zpx(bus, ram), // Zero Page, X
            AddrMode::ZPY => self.addr_zpy(bus, ram), // Zero Page, Y
            // AddrMode::REL => self.addr_rel(), // Relative
            AddrMode::ABS => self.addr_abs(bus, ram), // Absolute
            AddrMode::ABX => self.addr_abx(bus, ram), // Absolute, X
            AddrMode::ABY => self.addr_aby(bus, ram), // Absolute, Y
            // AddrMode::IND => self.addr_ind(), // Indirect
            AddrMode::INX => self.addr_inx(bus, ram), // Indirect, X
            AddrMode::INY => self.addr_iny(bus, ram), // Indirect, Y
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

    // Sets addr hi byte to pc + 1 and addr lo byte to pc
    fn addr_abs(&mut self, bus: &Bus, ram: &Ram) -> u16 {
        let addr = bus.read_u16(ram, self.pc as usize);
        self.set_pc(ProgramCounter::Skip);
        println!("Current Addr: {:04X}", addr);
        addr
    }

    // Sets addr to abs addr + x reg
    fn addr_abx(&mut self, bus: &Bus, ram: &Ram) -> u16 {
        let base_addr = bus.read_u16(ram, self.pc as usize);
        let addr = base_addr.wrapping_add(self.x as u16);
        self.set_pc(ProgramCounter::Skip);
        println!("Current Addr: {:04X}", addr);
        if (addr & 0xFF00) > (base_addr & 0xFF00) {
            self.page_crossed = true;
        }
        addr
    }

    // Sets addr to abs addr + y reg
    fn addr_aby(&mut self, bus: &Bus, ram: &Ram) -> u16 {
        let base_addr = bus.read_u16(ram, self.pc as usize);
        let addr = base_addr.wrapping_add(self.y as u16);
        self.set_pc(ProgramCounter::Skip);
        println!("Current Addr: {:04X}", addr);
        if (addr & 0xFF00) > (base_addr & 0xFF00) {
            self.page_crossed = true;
        }
        addr
    }

    // Sets addr to the addr held at the zpg redirected to by addr_lo at pc + x reg
    fn addr_inx(&mut self, bus: &Bus, ram: &Ram) -> u16 {
        let addr_lo = bus.read(ram, self.pc as usize).wrapping_add(self.x);
        self.set_pc(ProgramCounter::Next);
        let zpg_addr = (addr_lo as u16) & 0x00FF;
        println!("Zero PG Addr: {:04X}", zpg_addr);
        let addr = bus.read_u16(ram, zpg_addr as usize);
        println!("Current Addr: {:04X}", addr);
        addr
    }

    // Sets addr to the y reg + addr held at the zpg redirected from addr_lo at pc 
    fn addr_iny(&mut self, bus: &Bus, ram: &Ram) -> u16 {
        let addr_lo = bus.read(ram, self.pc as usize);
        self.set_pc(ProgramCounter::Next);
        let zpg_addr = (addr_lo as u16) & 0x00FF;
        println!("Zero PG Addr: {:04X}", zpg_addr);
        let base_addr = bus.read_u16(ram, zpg_addr as usize);
        let addr = base_addr + (self.y as u16);
        println!("Current Addr: {:04X}", addr);
        if (addr & 0xFF00) > (base_addr & 0xFF00) {
            self.page_crossed = true;
        }
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

            // Load/Store Operations
            0xA9 => self.opcode_lda(AddrMode::IMM, 2, bus, ram),
            0xA5 => self.opcode_lda(AddrMode::ZPG, 3, bus, ram),
            0xB5 => self.opcode_lda(AddrMode::ZPX, 4, bus, ram),
            0xAD => self.opcode_lda(AddrMode::ABS, 4, bus, ram),
            0xBD => self.opcode_lda(AddrMode::ABX, 4, bus, ram),
            0xB9 => self.opcode_lda(AddrMode::ABY, 4, bus, ram),
            0xA1 => self.opcode_lda(AddrMode::INX, 6, bus, ram),
            0xB1 => self.opcode_lda(AddrMode::INY, 5, bus, ram),

            _ => panic!("Unkown opcode {:X?} at PC {:X?}", current_opcode, self.pc),
        }
    }

    fn opcode_lda(&mut self, addr: AddrMode, cycles: u8, bus: &Bus, ram: &Ram) {
        let current_addr = self.fetch_addr(addr, bus, ram);
        self.a = bus.read(ram, current_addr as usize);
        self.set_flag_negative_zero(self.a);
        self.cycles += cycles as usize;
        if self.page_crossed {
            self.cycles += 1;
        }
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

    // Rework LDA test to be more succinct when done building
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
        assert_eq!(cpu.page_crossed, false);

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

        // Test LDA with ABS address
        bus.write(&mut ram, 0x0006, 0x05);
        bus.write(&mut ram, 0x0007, 0x04);
        bus.write(&mut ram, 0x0405, 0x46);
        cpu.opcode_lda(AddrMode::ABS, 4, &bus, &ram);
        assert_eq!(cpu.a, 0x46);

        // Test LDA with ABX address
        bus.write(&mut ram, 0x0008, 0x05);
        bus.write(&mut ram, 0x0009, 0x06);
        cpu.x = 0x05;
        bus.write(&mut ram, 0x060A, 0x38);
        cpu.opcode_lda(AddrMode::ABX, 4, &bus, &ram);
        assert_eq!(cpu.a, 0x38);
        assert_eq!(cpu.page_crossed, false);

        // Test LDA with ABY address
        bus.write(&mut ram, 0x000A, 0x06);
        bus.write(&mut ram, 0x000B, 0x07);
        cpu.y = 0x06;
        bus.write(&mut ram, 0x070C, 0x28);
        cpu.opcode_lda(AddrMode::ABY, 4, &bus, &ram);
        assert_eq!(cpu.a, 0x28);
        assert_eq!(cpu.page_crossed, false);

        // Test LDA with ABX address and page cross
        bus.write(&mut ram, 0x000C, 0x05);
        bus.write(&mut ram, 0x000D, 0x08);
        cpu.x = 0xFF;
        bus.write(&mut ram, 0x0904, 0xAA);
        cpu.opcode_lda(AddrMode::ABX, 4, &bus, &ram);
        assert_eq!(cpu.a, 0xAA);
        assert_eq!(cpu.cycles, 38);
        assert_eq!(cpu.page_crossed, true);

        // Test LDA with ABY address and page cross
        bus.write(&mut ram, 0x000E, 0x06);
        bus.write(&mut ram, 0x000F, 0x09);
        cpu.y = 0xFF;
        bus.write(&mut ram, 0x0A05, 0x26);
        cpu.opcode_lda(AddrMode::ABY, 4, &bus, &ram);
        assert_eq!(cpu.a, 0x26);
        assert_eq!(cpu.cycles, 43);
        assert_eq!(cpu.page_crossed, true);

        // Test LDA with INX address
        bus.write(&mut ram, 0x0010, 0xA4);
        cpu.x = 0x10;
        bus.write(&mut ram, 0x00B4, 0x04);
        bus.write(&mut ram, 0x00B5, 0x03);
        bus.write(&mut ram, 0x0304, 0x38);
        cpu.opcode_lda(AddrMode::INX, 6, &bus, &ram);
        assert_eq!(cpu.a, 0x38);

        // Test LDA with INY address
        bus.write(&mut ram, 0x0011, 0xA7);
        cpu.y = 0x02;
        bus.write(&mut ram, 0x00A7, 0x08);
        bus.write(&mut ram, 0x00A8, 0x02);
        bus.write(&mut ram, 0x020A, 0x19);
        cpu.opcode_lda(AddrMode::INY, 5, &bus, &ram);
        assert_eq!(cpu.a, 0x19);
        assert_eq!(cpu.page_crossed, false);
        assert_eq!(cpu.cycles, 54);

        // Test LDA with INY address and page cross
        bus.write(&mut ram, 0x0012, 0xA9);
        cpu.y = 0xFF;
        bus.write(&mut ram, 0x00A9, 0x08);
        bus.write(&mut ram, 0x00AA, 0x02);
        bus.write(&mut ram, 0x0307, 0x29);
        cpu.opcode_lda(AddrMode::INY, 5, &bus, &ram);
        assert_eq!(cpu.a, 0x29);
        assert_eq!(cpu.page_crossed, true);
        assert_eq!(cpu.cycles, 60);
    }
}