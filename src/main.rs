// Route ROM buffer to RAM mapping? (Is ROM buffer directly accessible...
//    or only accessible through RAM mapping?)
// Build out more detailed debug information

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
    let test_byte = bus.read(&ram, 0x7FF);
    println!("Test byte: {:X?}", test_byte);

    // Writing test byte to Ram
    bus.write(&mut ram, 0xFFF, 0xA1);

    // Ram test after writing to Ram
    let test_byte = bus.read(&ram, 0x7FF);
    println!("Test byte: {:X?}", test_byte);

    // CPU testing
    let test_pc = cpu.tick(&bus, &ram);
    println!("Test pc: {:X?}", test_pc);

    // ROM load test (Currently limited to the beginning of a file)
    println!("ROM file name: {:X?}", &rom.buffer[0..480]);
}