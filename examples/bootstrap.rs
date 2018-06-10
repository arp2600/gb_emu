extern crate gb_emu;
use gb_emu::Emulator;
use std::fs;

fn main() {
    let boot_rom = "../ROMs/dmg_rom.gb";
    let cartridge_rom = "../cpu_instrs/cpu_instrs.gb";
    let mut emulator = Emulator::new(Some(&boot_rom), cartridge_rom);
    while emulator.is_boot_rom_enabled() {
        emulator.tick();
    }
    println!("boot rom finished");
    emulator.set_tracing(true);
    loop {
        emulator.tick()
    }
}
