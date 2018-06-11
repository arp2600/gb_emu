extern crate gb_emu;
use gb_emu::Emulator;

fn main() {
    let cartridge_rom = "blargg_test_roms/cpu_instrs/individual/01-special.gb";
    // let cartridge_rom = "blargg_test_roms/cpu_instrs/individual/02-interrupts.gb";
    let mut emulator = Emulator::new(None, cartridge_rom);
    let mut serial_vector = Vec::new();

    let serial_callback = move |x| {
        serial_vector.push(x);
        let mut s = String::new();
        for (i, &c) in serial_vector.iter().enumerate() {
            if i % 2 == 0 {
                s.push(c as char);
            }
        }
        println!("serial data: \"{}\"", s);
    };

    emulator.set_serial_io_callback(Box::new(serial_callback));

    for _ in 0..300_000 {
        emulator.tick();
        if emulator.get_registers().pc == 51154 {
            println!("end of test at pc 51154");
            break;
        }
    }
    emulator.set_tracing(true);
    for _ in 0..5 {
        emulator.tick();
    }
}
