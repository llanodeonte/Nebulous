mod cpu;
mod bus;
mod ram;

use cpu::Cpu;
use bus::Bus;
use ram::Ram;

fn main() {
    let cpu = Cpu::new();
    let bus = Bus::new();
    let mut ram = Ram::new();

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
}