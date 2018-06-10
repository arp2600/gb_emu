mod cartridge;
mod cpu;
mod lcd;
mod lcd_registers;
mod memory;
mod memory_values;
mod opcode_table;
mod registers;
use cartridge::Cartridge;
use cpu::Cpu;
use lcd::LCD;
use memory::Memory;
use registers::Registers;
use std::fs;

pub struct Emulator {
    cpu: Cpu,
    lcd: LCD,
    memory: Memory,
    tracing: bool,
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

        Emulator {
            cpu,
            lcd,
            memory,
            tracing: false,
        }
    }

    pub fn tick(&mut self) {
        self.lcd.tick(&mut self.memory, self.cpu.get_cycles());
        self.cpu.tick(&mut self.memory, self.tracing);
    }

    pub fn is_boot_rom_enabled(&self) -> bool {
        self.memory.is_boot_rom_enabled()
    }

    pub fn get_registers(&self) -> &Registers {
        self.cpu.get_registers()
    }

    pub fn set_tracing(&mut self, state: bool) {
        self.tracing = state;
    }

    pub fn set_serial_io_callback(&mut self, callback: Box<FnMut(u8)>) {
        self.memory.set_serial_io_callback(callback);
    }
}
