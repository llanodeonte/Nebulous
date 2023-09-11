mod cpu;
mod bus;
mod ram;

use std::env;
use std::fs;
use std::io::Read;

use cpu::Cpu;
use bus::Bus;
use ram::Ram;

fn main() {
    let cpu = Cpu::new();
    let bus = Bus::new();
    let mut ram = Ram::new();

    // Load and store ROM data
    let rom_file_path = env::args().nth(1).unwrap();
    let mut rom_file = fs::File::open(rom_file_path).unwrap();
    let mut rom_data = Vec::new();
    rom_file.read_to_end(&mut rom_data).unwrap();

    // Ram test prior to writing to Ram
    let test_byte = bus.read_ram(&ram, 0xFF);
    println!("Test byte: {:X?}", test_byte);

    // Writing test byte to Ram
    bus.write_ram(&mut ram, 0xFF, 0xA1);

    // Ram test after writing to Ram
    let test_byte = bus.read_ram(&ram, 0xFF);
    println!("Test byte: {:X?}", test_byte);

    // CPU testing
    let test_pc = cpu.tick(&bus, &ram);
    println!("Test pc: {:X?}", test_pc);

    // ROM load test
    println!("ROM file name: {:X?}", rom_data);
}