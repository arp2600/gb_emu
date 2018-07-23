extern crate gb_emu;
use gb_emu::{App, Command, Emulator, JoyPad};
use std::str;

struct DummyApp {}
impl App for DummyApp {
    fn draw_line(&mut self, line_buffer: &[u8], line_index: u8) {}
    fn update(&mut self, joypad: &mut JoyPad) -> Command {
        return Command::Stop;
    }
}

fn main() {
    // let cartridge_rom = "blargg_test_roms/cpu_instrs/individual/01-special.gb";
    // let cartridge_rom = "blargg_test_roms/cpu_instrs/individual/02-interrupts.gb";
    // let cartridge_rom = "blargg_test_roms/cpu_instrs/individual/06-ld_r_r.gb";
    let cartridge_rom = "../ROMs/tetris.gb";
    let mut emulator = Emulator::new(None, cartridge_rom);

    emulator.set_tracing(true);
    let mut app = DummyApp {};
    for _ in 0..2_000_000 {
        emulator.tick(&mut app);
    }

    let serial_data = emulator.get_serial_data();
    let serial_string = str::from_utf8(serial_data).unwrap();
    println!("{}", serial_string);
}
