mod cartridge;
mod cpu;
mod lcd_registers;
mod memory;
mod opcode_table;
mod opcodes;
mod registers;
use cartridge::Cartridge;
use cpu::Cpu;
use memory::Memory;
use registers::Registers;
use std::fs;

fn get_boot_rom() -> Vec<u8> {
    fs::read("../ROMs/dmg_rom.gb").unwrap()
}

struct LCD {
    enabled: bool,
    draw_time: u64,
}

impl LCD {
    fn new() -> LCD {
        LCD { enabled: false, draw_time: 0 }
    }

    fn tick(&mut self, cpu_cycles: u64, memory: &mut Memory) {
        if self.enabled {
            self.run(cpu_cycles, memory);
        } else {
            // check if enabled now
        }
    }

    fn run(&mut self, cpu_cycles: u64, memory: &mut Memory) {
        // do stuff
    }
}

fn main() {
    let mut boot_rom = get_boot_rom();
    let mut registers = Registers::new();
    let mut cartridge = Cartridge::from_file("../ROMs/tetris.gb");
    let mut mem = Memory::new(&mut boot_rom, &mut cartridge);
    let mut cpu = Cpu::new(&mut registers);
    let mut lcd = LCD::new();

    loop {
        cpu.tick(&mut mem);
        lcd.tick(cpu.get_cycles(), &mut mem);
    }
}
