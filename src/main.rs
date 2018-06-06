mod cartridge;
mod cpu;
mod lcd;
mod lcd_registers;
mod memory;
mod opcode_table;
mod opcodes;
mod registers;
use cartridge::Cartridge;
use cpu::Cpu;
use lcd::LCD;
use memory::Memory;
use registers::Registers;
use std::fs;

fn get_boot_rom() -> Vec<u8> {
    fs::read("../ROMs/dmg_rom.gb").unwrap()
}

fn main() {
    let mut boot_rom = get_boot_rom();
    let mut registers = Registers::new();
    let mut cartridge = Cartridge::from_file("../ROMs/tetris.gb");
    let mut mem = Memory::new(&mut boot_rom, &mut cartridge);
    let mut cpu = Cpu::new(&mut registers);
    let mut lcd = LCD::new();

    loop {
        lcd.tick(&mut mem, cpu.get_cycles());
        cpu.tick(&mut mem);
    }
}
