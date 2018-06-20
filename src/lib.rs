mod bit_ops;
mod cartridge;
mod cpu;
mod lcd;
mod lcd_registers;
mod memory;
mod memory_values;
mod opcode_table;
mod registers;
mod timer;
use cartridge::Cartridge;
use cpu::Cpu;
use lcd::LCD;
use memory::Memory;
use registers::Registers;
use std::fs;
use timer::Timer;

pub struct Emulator {
    cpu: Cpu,
    lcd: LCD,
    memory: Memory,
    timer: Timer,
    tracing: bool,
}

impl Emulator {
    pub fn new(boot_rom_path: Option<&str>, cartridge_rom_path: &str) -> Emulator {
        let cpu = Cpu::new();
        let cartridge = Cartridge::from_file(cartridge_rom_path);
        let boot_rom = match boot_rom_path {
            Some(x) => fs::read(x).unwrap(),
            None => {
                let x = include_bytes!("../resources/dummy_boot_rom.gb");
                x.to_vec()
            }
        };
        let memory = Memory::new(boot_rom, cartridge);
        let lcd = LCD::new();

        Emulator {
            cpu,
            lcd,
            memory,
            timer: Timer::new(),
            tracing: false,
        }
    }

    pub fn tick(&mut self) {
        self.lcd.tick(&mut self.memory, self.cpu.get_cycles());
        self.cpu.tick(&mut self.memory, self.tracing);
        self.timer.tick(&mut self.memory, self.cpu.get_cycles());
        self.cpu.check_interrupts(&mut self.memory);
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

    pub fn get_serial_data(&self) -> &[u8] {
        self.memory.get_serial_data()
    }
}
