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

    cpu.x = 0x01;
    cpu.y = 0x02;

    // Load test bytes to RAM
    // Will change main loop testing method after full LDA implementation
    bus.write(&mut ram, 0x0000, 0xA9); // LDA IMM
    bus.write(&mut ram, 0x0001, 0x2B); // Random data
    bus.write(&mut ram, 0x0002, 0xA5); // LDA ZPG
    bus.write(&mut ram, 0x0003, 0x24); // ZPG addr
    bus.write(&mut ram, 0x0024, 0x57); // Random data
    bus.write(&mut ram, 0x0004, 0xB5); // LDA ZPX
    bus.write(&mut ram, 0x0005, 0x1F); // ZPX base addr
    bus.write(&mut ram, 0x0020, 0x34); // Random data
    bus.write(&mut ram, 0x0006, 0xAD); // LDA ABS
    bus.write(&mut ram, 0x0007, 0x27); // ABS addr lo
    bus.write(&mut ram, 0x0008, 0x00); // ABS addr hi
    bus.write(&mut ram, 0x0027, 0x17); // Random data
    bus.write(&mut ram, 0x0009, 0xBD); // LDA ABX
    bus.write(&mut ram, 0x000A, 0x2C); // ABX base addr lo
    bus.write(&mut ram, 0x000B, 0x00); // ABX base addr hi
    bus.write(&mut ram, 0x002D, 0x27); // Random data
    bus.write(&mut ram, 0x000C, 0xB9); // LDA ABY
    bus.write(&mut ram, 0x000D, 0x2A); // ABY base addr lo
    bus.write(&mut ram, 0x000E, 0x00); // ABY base addr hi
    bus.write(&mut ram, 0x002C, 0x14); // Random data
    bus.write(&mut ram, 0x000F, 0xBD); // LDA ABX
    bus.write(&mut ram, 0x0010, 0xFF); // ABX base addr lo
    bus.write(&mut ram, 0x0011, 0x01); // ABX base addr hi
    bus.write(&mut ram, 0x0200, 0xAC); // Random data
    bus.write(&mut ram, 0x0012, 0xB9); // LDA ABY
    bus.write(&mut ram, 0x0013, 0xFF); // ABY base addr lo
    bus.write(&mut ram, 0x0014, 0x01); // ABY base addr hi
    bus.write(&mut ram, 0x0201, 0xBF); // Random data
    bus.write(&mut ram, 0x0015, 0xA1); // LDA INX
    bus.write(&mut ram, 0x0016, 0xA4); // INX zpg addr lo
    bus.write(&mut ram, 0x00A5, 0x04); // INX addr lo
    bus.write(&mut ram, 0x00A6, 0x03); // INX addr hi
    bus.write(&mut ram, 0x0304, 0x38); // Random data
    bus.write(&mut ram, 0x0017, 0xB1); // LDA INY
    bus.write(&mut ram, 0x0018, 0xC7); // INY zpg addr lo
    bus.write(&mut ram, 0x00C7, 0xFF); // INY addr lo
    bus.write(&mut ram, 0x00C8, 0x03); // INY addr hi
    bus.write(&mut ram, 0x0401, 0x29); // Random data


    let mut loop_limit = 10;

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