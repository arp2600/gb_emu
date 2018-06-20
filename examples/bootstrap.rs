extern crate gb_emu;
use gb_emu::Emulator;
use std::str;

fn main() {
    let cartridge_rom = "../ROMs/tetris.gb";
    let boot_rom = "../ROMs/dmg_rom.gb";
    let mut emulator = Emulator::new(Some(boot_rom), cartridge_rom);

    // emulator.set_tracing(true);
    for _ in 0..3_000_000 {
        emulator.tick();
    }

    let serial_data = emulator.get_serial_data();
    let serial_string = str::from_utf8(serial_data).unwrap();
    println!("{}", serial_string);
}
