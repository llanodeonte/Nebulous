// Route ROM buffer to RAM mapping? (Is ROM buffer directly accessible...
//    or only accessible through RAM mapping?)
// Build out more detailed debug information
// Rework main to work with new CPU pipeline for testing

mod cpu;
mod bus;
mod ram;
mod rom;

use cpu::Cpu;
use bus::Bus;
use ram::Ram;
use rom::Rom;

fn main() {
    let mut cpu = Cpu::new();
    let bus = Bus::new();
    let mut ram = Ram::new();
    let mut rom = Rom::new();
    rom.load_rom();

    // Load test bytes to RAM
    bus.write(&mut ram, 0x0000, 0xA9); // LDA IMM
    bus.write(&mut ram, 0x0001, 0x2B); // Random data
    bus.write(&mut ram, 0x0002, 0xA5); // LDA ZPG
    bus.write(&mut ram, 0x0003, 0xC4); // ZPG Addr
    bus.write(&mut ram, 0x00C4, 0x57); // Random data

    let mut loop_limit = 2;

    cpu.debug_print();

    while loop_limit > 0 {
        if cpu.cycles == 0 {
            println!("Cycles: {:?}", cpu.cycles);
            println!("------------------");
            cpu.clock(&bus, &ram);
            cpu.debug_print();
            loop_limit -= 1;
        }
        println!("Cycles: {:?}", cpu.cycles);
        cpu.cycles -= 1;
    }
}