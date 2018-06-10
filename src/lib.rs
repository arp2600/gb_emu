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
use registers::Registers;

pub struct Emulator {
    cpu: Cpu,
    lcd: LCD,
    memory: Memory,
}

impl Emulator {
    pub fn new(boot_rom_path: Option<&str>, cartridge_rom_path: &str) -> Emulator {
        let mut cpu = Cpu::new();
        let cartridge = Cartridge::from_file(cartridge_rom_path);

        let memory = if let Some(x) = boot_rom_path {
            let boot_rom = fs::read(x).ok();
            Memory::new(boot_rom, cartridge)
        } else {
            cpu.skip_boot_rom();
            let mut m = Memory::new(None, cartridge);
            m.skip_boot_rom();
            m
        };
        let lcd = LCD::new();

        Emulator { cpu, lcd, memory }
    }

    pub fn tick(&mut self) {
        self.lcd.tick(&mut self.memory, self.cpu.get_cycles());
        self.cpu.tick(&mut self.memory);
    }

    pub fn is_boot_rom_enabled(&self) -> bool {
        self.memory.is_boot_rom_enabled()
    }

    pub fn get_registers(&self) -> &Registers {
        self.cpu.get_registers()
    }
}
