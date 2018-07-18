extern crate gb_emu;
use gb_emu::{Command, Emulator, JoyPad};
use std::time::{Duration, Instant};

fn main() {
    let durations = {
        let mut v = Vec::new();
        for _ in 0..3 {
            let d = run_test();
            v.push(d);
        }
        v
    };
    let total: Duration = durations.iter().sum();
    let average = total / 3;
    // println!("_BENCH_ durations: {:?}", durations);
    // println!("_BENCH_ total: {:?}", total);
    // println!("_BENCH_ average: {:?}", average);

    println!("_BENCH_ rom: cpu_instrs.gb");
    println!("_BENCH_ average: {:?}", average);
}

fn run_test() -> Duration {
    let cartridge_rom = "gb-test-roms/cpu_instrs/cpu_instrs.gb";
    let mut emulator = Emulator::new(None, cartridge_rom);

    emulator.set_tracing(false);
    let update_fn = {
        let mut frame_counter = 0;

        move |_: &mut JoyPad| {
            frame_counter += 1;
            if frame_counter >= 3350 {
                Command::Stop
            } else {
                Command::Continue
            }
        }
    };

    let start = Instant::now();
    emulator.run(|_, _| {}, update_fn);
    start.elapsed()
}
