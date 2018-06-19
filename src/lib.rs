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
use bit_ops::BitGetSet;
use cartridge::Cartridge;
use cpu::Cpu;
use lcd::LCD;
use memory::Memory;
use memory_values::IoRegs;
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
        self.check_for_interrupts();
    }

    pub fn check_for_interrupts(&mut self) {
        let interrupt_request = self.memory.get_u8(IoRegs::IF as u16);
        let interrupt_enable = self.memory.get_u8(IoRegs::IE as u16);
        let interrupts = interrupt_request & interrupt_enable;
        if interrupts.get_bit(2) {
            if self.cpu.try_interrupt(0x50, &mut self.memory) {
                let flag = interrupt_request.reset_bit(2);
                self.memory.set_u8(IoRegs::IF as u16, flag);
            }
        }
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
