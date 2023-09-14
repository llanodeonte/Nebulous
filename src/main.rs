mod cpu;
mod bus;
mod ram;
mod rom;

use cpu::Cpu;
use bus::Bus;
use ram::Ram;
use rom::Rom;

fn main() {
    let cpu = Cpu::new();
    let bus = Bus::new();
    let mut ram = Ram::new();
    let mut rom = Rom::new();
    rom.load_rom();

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

    // ROM load test (Currently limited to the beginning of a file)
    println!("ROM file name: {:X?}", &rom.buffer[0..480]);
}