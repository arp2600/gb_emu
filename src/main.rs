mod cpu;
mod memory;
mod opcode_table;
mod opcodes;
mod registers;
use cpu::Cpu;
use memory::Memory;
use registers::Registers;
use std::fs;

fn get_boot_rom() -> Vec<u8> {
    fs::read("../ROMs/dmg_rom.gb").unwrap()
}

fn main() {
    let mut boot_rom = get_boot_rom();
    let mut registers = Registers::new();
    let mut mem = Memory::new(&mut boot_rom);
    let mut cpu = Cpu::new(&mut registers, &mut mem);

    loop {
        cpu.tick();
    }
}
