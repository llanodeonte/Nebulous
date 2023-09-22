// Route ROM buffer to RAM mapping? (Is ROM buffer directly accessible...
//    or only accessible through RAM mapping?)

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
    bus.write(&mut ram, 0x0003, 0xC4); // ZPG addr
    bus.write(&mut ram, 0x00C4, 0x57); // Random data
    bus.write(&mut ram, 0x0004, 0xB5); // LDA ZPX
    bus.write(&mut ram, 0x0005, 0x1F); // ZPX base addr
    cpu.x = 0x10;                                 // ZPX offset
    bus.write(&mut ram, 0x002F, 0x34); // Random data
    bus.write(&mut ram, 0x0006, 0xAD); // LDA ABS
    bus.write(&mut ram, 0x0007, 0x47); // ABS addr lo
    bus.write(&mut ram, 0x0008, 0x01); // ABS addr hi
    bus.write(&mut ram, 0x0147, 0x17); // Random data
    
    let mut loop_limit = 4;

    cpu.debug_print();

    while loop_limit > 0 {
        if cpu.cycles == 0 {
            println!("Cycles: {:?}, Executing", cpu.cycles);
            println!("--------------------");
            cpu.clock(&bus, &ram);
            cpu.debug_print();
            loop_limit -= 1;
        }
        println!("Cycles: {:?}", cpu.cycles);
        cpu.cycles -= 1;
    }
}