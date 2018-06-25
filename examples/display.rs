extern crate gb_emu;
use gb_emu::Emulator;
use gb_emu::Screen;
use std::io::{self, Write};
use std::str;

struct DisplayPipe {}

impl Screen for DisplayPipe {
    fn write_line(&mut self, ly: u8, pixels: &[u8; 160]) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        handle.write(b"LINE").unwrap();
        write!(handle, " {} ", ly).unwrap();
        let mut line = [35; 160];
        for (i, pixel) in pixels.iter().enumerate() {
            line[i as usize] = (pixel + '!' as u8) as u8;
        }
        handle.write(&line).unwrap();
        handle.write(b"\n").unwrap();
    }

    fn end_frame(&mut self) {}
}

fn main() {
    let cartridge_rom = "../ROMs/tetris.gb";
    let boot_rom = "../ROMs/dmg_rom.gb";
    let mut emulator = Emulator::new(Some(boot_rom), cartridge_rom);
    // let mut emulator = Emulator::new(None, cartridge_rom);
    let mut display = DisplayPipe {};

    println!("Starting emulator");
    // emulator.set_tracing(true);
    for i in 0..5_000_000 {
        if i % 1000000 == 0 {
            println!("{} ticks", i)
        }
        emulator.tick(Some(&mut display));
    }

    let serial_data = emulator.get_serial_data();
    let serial_string = str::from_utf8(serial_data).unwrap();
    println!("{}", serial_string);
}
