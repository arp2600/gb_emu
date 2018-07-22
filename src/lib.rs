#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;
#[macro_use]
mod warn_macros;
mod bit_ops;
mod cartridge;
mod cpu;
mod lcd;
mod memory;
mod opcode_table;
mod registers;
mod timer;
use cartridge::Cartridge;
use cpu::Cpu;
use lcd::LCD;
pub use memory::JoyPad;
use memory::Memory;
use registers::Registers;
use std::fs;
use timer::Timer;

pub trait App {
    fn draw_line(&mut self, line_buffer: &[u8], line_index: u8);
    fn update(&mut self, joypad: &mut JoyPad) -> Command;
}

pub enum Command {
    Continue,
    Stop,
}

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

    pub fn run<T: App>(&mut self, app: &mut T) {
        loop {
            while !self.lcd.is_vblank() {
                self.tick(app);
            }
            self.lcd.reset_vblank();
            let joypad = self.memory.get_joypad();
            match app.update(joypad) {
                Command::Stop => break,
                Command::Continue => (),
            }
        }
    }

    pub fn tick<T: App>(&mut self, app: &mut T) {
        {
            let vram = self.memory.get_video_memory();
            self.lcd.tick(vram, self.cpu.get_cycles(), app);
        }
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

    pub fn get_serial_data(&self) -> &[u8] {
        self.memory.get_serial_data()
    }
}
