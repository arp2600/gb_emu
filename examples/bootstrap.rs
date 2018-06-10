extern crate gb_emu;
use gb_emu::Emulator;
use std::fs;

fn main() {
    let cartridge_rom = "blargg_test_roms/cpu_instrs/individual/01-special.gb";
    let mut emulator = Emulator::new(None, cartridge_rom);
    let mut serial_vector = Vec::new();

    let serial_callback = move |x| {
        serial_vector.push(x);
        println!("serial data: {:?}", serial_vector);
    };

    emulator.set_serial_io_callback(Box::new(serial_callback));

    // emulator.set_tracing(true);
    loop {
        emulator.tick();
    }
}
