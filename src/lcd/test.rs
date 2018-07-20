// extern crate png_encode_mini;
use super::LCD;
use cartridge::Cartridge;
use memory::locations::*;
use memory::{io_regs, Memory, VideoMemory};
// use self::png_encode_mini::write_rgba_from_u8;
// use std::fs::File;

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

#[test]
fn bg_checker_pattern() {
    let mut vmem = VideoMemory::test_new();
    vmem.regs.lcdc = 0b1001_0000;
    vmem.regs.bgp = 0b00_01_10_11;

    color_tile(TILE_DATA_2, 0, 0, &mut vmem);
    color_tile(TILE_DATA_2, 1, 3, &mut vmem);

    for i in 0..1024 {
        vmem[TILE_MAP_1 as usize + i] = (((i % 2) + (i / 32)) % 2) as u8;
    }

    let mut lcd = LCD::new();

    let mut buffer = [0; 160 * 144];

    // Run 1 frame
    for cycles in 0..70224 {
        lcd.tick(&mut vmem, cycles, |line, line_index| {
            for (i, v) in line.iter().enumerate() {
                buffer[usize::from(line_index) * 160 + i] = *v;
            }
        });
    }

    for (i, actual_value) in buffer.iter().enumerate() {
        let ii: usize = i / 8;
        let desired_value = (((ii % 2) + (ii / 160)) % 2) as u8;
        assert_eq!(desired_value * 3, *actual_value, "i is {}", i);
    }
}

fn color_tile(data_start: u16, index: usize, value: u8, vmem: &mut VideoMemory) {
    assert!(value < 4);
    let data_start = data_start as usize + index * 16;
    for i in 0..8 {
        vmem[data_start + i * 2] = if value & 0b01 != 0 { 0xff } else { 0 };

        vmem[data_start + i * 2 + 1] = if value & 0b10 != 0 { 0xff } else { 0 };
    }
}

// fn write_png(fname: &str, buffer: &[u8; 160*144]) {
//     let mut rgba_buffer = [0; 160*144*4];
//
//     for (i, v) in buffer.iter().enumerate() {
//         let v = *v * 85;
//         rgba_buffer[i * 4] = v;
//         rgba_buffer[i * 4 + 1] = v;
//         rgba_buffer[i * 4 + 2] = v;
//         rgba_buffer[i * 4 + 3] = 255;
//     }
//
//     let mut file = File::create(fname).unwrap();
//     write_rgba_from_u8(&mut file, &rgba_buffer, 160, 144).unwrap();
//     println!("saved image {}", fname);
// }
