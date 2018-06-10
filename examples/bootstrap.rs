extern crate gb_emu;
use gb_emu::Emulator;
use std::fs;

fn main() {
    let boot_rom = "../ROMs/dmg_rom.gb";
    let cartridge_rom = "../ROMs/tetris.gb";
    let mut emulator = Emulator::new(Some(&boot_rom), cartridge_rom);
    while emulator.is_boot_rom_enabled() {
        emulator.tick();
    }
    println!("boot rom finished");
    loop {
        emulator.tick()
    }
}
