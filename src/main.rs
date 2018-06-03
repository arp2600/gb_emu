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
    frame_start_time: u64,
}

impl LCD {
    fn new() -> LCD {
        LCD { enabled: false, frame_start_time: 0 }
    }

    fn tick(&mut self, cpu_cycles: u64, memory: &mut Memory) {
        if self.enabled {
            self.run(cpu_cycles, memory);
        } else {
            // check if enabled now
            if memory.get_u8(0xff40) & 0b1000_0000 != 0 {
                self.enabled = true;
                self.frame_start_time = cpu_cycles;
            }
        }
    }

    fn run(&mut self, cpu_cycles: u64, memory: &mut Memory) {
        let run_time = cpu_cycles - self.frame_start_time;
        if run_time < 15800 {
            // write to sy
            memory.set_u8(0xff44, 0);
        } else {
            let ly = (run_time - 15800) / 480 + 1;
            memory.set_u8(0xff44, ly as u8);
        }
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
