use super::LCD;
use cartridge::Cartridge;
use memory::{io_regs, Memory};

// Check lcd against old algorithm for calculating ly register
#[test]
fn ly_timing() {
    let mut memory = {
        let boot_rom = Vec::new();
        let cartridge = Cartridge::create_dummy();
        Memory::new(boot_rom, cartridge)
    };

    let mut lcd = LCD::new();

    memory.set_io(io_regs::LCDC, 0b1000_0000);
    // Run for 10 frames
    for cycles in 0..(70224 * 10) {
        let frame_time = cycles % (456 * 154);
        let test_ly = (frame_time / 456) as u8;

        lcd.tick(memory.get_video_memory(), cycles, |_, _| {});
        let lcd_ly = memory.get_io(io_regs::LY);

        assert_eq!(
            lcd_ly, test_ly,
            "lcd {} != test {} at cycles {}",
            lcd_ly, test_ly, cycles
        );
    }
}

// Check stat mode timing matches
// timing described in
//   The Cycle-Accurate
//   Game Boy Docs
//   page 32
#[test]
fn stat_mode_timing() {
    let mut memory = {
        let boot_rom = Vec::new();
        let cartridge = Cartridge::create_dummy();
        Memory::new(boot_rom, cartridge)
    };

    let mut lcd = LCD::new();

    memory.set_io(io_regs::LCDC, 0b1000_0000);
    {
        let stat = memory.get_io(io_regs::STAT);
        let ly = memory.get_io(io_regs::LY);
        assert_eq!(stat & 0b11, 0);
        assert_eq!(ly, 0);
    }

    for frame in 0..10 {
        test_stat_mode_frame(&mut lcd, &mut memory, frame);
    }
}

fn test_stat_mode_frame(lcd: &mut LCD, memory: &mut Memory, frame_num: u64) {
    let base_cycles = frame_num * 154 * 456;
    // Test line 0 to 143 timings
    for line in 0..144 {
        let cycles = line * 456 + base_cycles;
        // Mode 0 for first 4 cycles
        for c in cycles..(cycles + 4) {
            lcd.tick(memory.get_video_memory(), c, |_, _| {});
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 0);
            assert_eq!(ly as u64, line);
        }
        // Test line 0 timings
        for c in (cycles + 4)..(cycles + 84) {
            lcd.tick(memory.get_video_memory(), c, |_, _| {});
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 2);
            assert_eq!(ly as u64, line);
        }
        {
            // Mode 3 for indefinate time starting at 84
            lcd.tick(memory.get_video_memory(), cycles + 84, |_, _| {});
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 3);
            assert_eq!(ly as u64, line);
        }
        // By 448, mode should be 0, and should remain till end of scanline
        for c in (cycles + 448)..(cycles + 456) {
            lcd.tick(memory.get_video_memory(), c, |_, _| {});
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 0);
            assert_eq!(ly as u64, line);
        }
    }
    // Test line 144 timings
    {
        let line = 144;
        let cycles = line * 456 + base_cycles;
        // Mode 0 for first 4 cycles
        for c in cycles..(cycles + 4) {
            lcd.tick(memory.get_video_memory(), c, |_, _| {});
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 0);
            assert_eq!(ly as u64, line);
        }
        // Mode 1 for remaining cycles
        for c in (cycles + 4)..(cycles + 456) {
            lcd.tick(memory.get_video_memory(), c, |_, _| {});
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 1);
            assert_eq!(ly as u64, line);
        }
    }
    // Test line 145 to 153 timings
    for line in 145..=153 {
        let cycles = line * 456 + base_cycles;
        // Mode 1 for all cycles
        for c in cycles..(cycles + 456) {
            lcd.tick(memory.get_video_memory(), c, |_, _| {});
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 1);
            assert_eq!(ly as u64, line);
        }
    }
}
