mod cpu;
mod bus;
mod ram;

use cpu::Cpu;
use bus::Bus;
use ram::Ram;

fn main() {
    let cpu = Cpu::new();
    let mut bus = Bus::new();
    let mut ram = Ram::new();

    // Ram testing
    let test_byte = bus.read_ram(&mut ram, 0xFF);
    println!("Test byte: {:X?}", test_byte);

    bus.write_ram(&mut ram, 0xFF, 0xA1);

    let test_byte = bus.read_ram(&mut ram, 0xFF);
    println!("Test byte: {:X?}", test_byte);

    // CPU testing
    let test_pc = cpu.fetch_opcode(&mut bus, &mut ram);
    println!("Test pc: {:X?}", test_pc);
}