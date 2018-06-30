extern crate gb_emu;
use gb_emu::Emulator;
use std::str;

#[test]
fn interrupt_time() {
    let test_rom_path = "gb-test-roms/interrupt_time/interrupt_time.gb";

    let mut emulator = Emulator::new(None, test_rom_path);
    for _ in 0..10_000_000 {
        emulator.tick(|_, _| {})
    }

    let serial_data = emulator.get_serial_data();
    let serial_string = str::from_utf8(serial_data).unwrap();
    assert!(
        serial_string.find("Passed").is_some(),
        "Test rom failed with message '{}'",
        serial_string
    );
}
