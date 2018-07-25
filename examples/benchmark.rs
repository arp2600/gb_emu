extern crate gb_emu;
use gb_emu::{App, Command, Emulator, JoyPad};
use std::time::{Duration, Instant};

struct BenchmarkApp {
    frame_counter: u64,
    run_till: u64,
}

impl App for BenchmarkApp {
    fn draw_line(&mut self, _: &[u8], _: u8) {}
    fn update(&mut self, _: &mut JoyPad) -> Command {
        self.frame_counter += 1;
        if self.frame_counter >= self.run_till {
            Command::Stop
        } else {
            Command::Continue
        }
    }
}

fn main() {
    let rom = "../../ROMs/super_mario_land_1.1.gb";
    println!("_BENCH_ rom: {}", rom);
    let durations = {
        let mut v = Vec::new();
        for i in 0..10 {
            let d = run_test(rom, 683, 1973);
            v.push(d);
            println!("_BENCH_ run_{}: {:?}", i, d);
        }
        v
    };
    let total: Duration = durations.iter().sum();
    let average = total / durations.len() as u32;

    println!("_BENCH_ average: {:?}", average);
}

fn run_test(cart: &str, start_at_frames: u64, end_at_frames: u64) -> Duration {
    let mut emulator = Emulator::new(None, cart);
    let mut app = BenchmarkApp {
        frame_counter: 0,
        run_till: start_at_frames,
    };
    emulator.run(&mut app);
    assert_eq!(app.frame_counter, start_at_frames);

    app.run_till = end_at_frames;
    let start = Instant::now();
    emulator.run(&mut app);
    let test_duration = start.elapsed();
    assert_eq!(app.frame_counter, end_at_frames);
    test_duration
}
