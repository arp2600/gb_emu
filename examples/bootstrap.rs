extern crate gb_emu;
use gb_emu::Emulator;
use std::fs;

fn main() {
    let boot_rom = "../ROMs/dmg_rom.gb";
    let cartridge_rom = "../ROMs/tetris.gb";
    let mut emulator = Emulator::new(&boot_rom, &cartridge_rom);
    loop {
        emulator.tick();
    }
}
