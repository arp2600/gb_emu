extern crate gb_emu;
use gb_emu::Emulator;
use std::str;

fn main() {
    // let cartridge_rom = "blargg_test_roms/cpu_instrs/individual/01-special.gb";
    // let cartridge_rom = "blargg_test_roms/cpu_instrs/individual/02-interrupts.gb";
    // let cartridge_rom = "blargg_test_roms/cpu_instrs/individual/06-ld_r_r.gb";
    let cartridge_rom = "blargg_test_roms/cpu_instrs/individual/05-op_rp.gb";
    let mut emulator = Emulator::new(None, cartridge_rom);

    // emulator.set_tracing(true);
    for _ in 0..3_000_000 {
        emulator.tick();
    }

    let serial_data = emulator.get_serial_data();
    let serial_string = str::from_utf8(serial_data).unwrap();
    println!("{}", serial_string);
}
