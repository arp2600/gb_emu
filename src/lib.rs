mod cartridge;
mod cpu;
mod lcd;
mod lcd_registers;
mod memory;
mod opcode_table;
mod registers;
mod memory_values;
use cartridge::Cartridge;
use cpu::Cpu;
use lcd::LCD;
use memory::Memory;
use std::fs;

pub struct Emulator {
    cpu: Cpu,
    lcd: LCD,
    memory: Memory,
}

impl Emulator {
    pub fn new(boot_rom_path: &str, cartridge_rom_path: &str) -> Emulator {
        let boot_rom = fs::read(boot_rom_path).unwrap();
        let mut cartridge = Cartridge::from_file(cartridge_rom_path);
        let mut memory = Memory::new(boot_rom, cartridge);
        let mut cpu = Cpu::new();
        let mut lcd = LCD::new();

        Emulator { cpu, lcd, memory }
    }

    pub fn tick(&mut self) {
        self.lcd.tick(&mut self.memory, self.cpu.get_cycles());
        self.cpu.tick(&mut self.memory);
    }
}
