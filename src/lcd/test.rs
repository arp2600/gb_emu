extern crate png_encode_mini;
use self::png_encode_mini::write_rgba_from_u8;
use super::LCD;
use cartridge::Cartridge;
use memory::locations::*;
use memory::{io_regs, JoyPad, Memory, VideoMemory};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read, Write};
use {App, Command};

struct DummyApp {}

impl App for DummyApp {
    fn draw_line(&mut self, line_buffer: &[u8], line_index: u8) {}
    fn update(&mut self, joypad: &mut JoyPad) -> Command {
        return Command::Stop;
    }
}

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

        let mut app = DummyApp {};
        lcd.tick(memory.get_video_memory(), cycles, &mut app);
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
            let mut app = DummyApp {};
            lcd.tick(memory.get_video_memory(), c, &mut app);
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 0);
            assert_eq!(ly as u64, line);
        }
        // Test line 0 timings
        for c in (cycles + 4)..(cycles + 84) {
            let mut app = DummyApp {};
            lcd.tick(memory.get_video_memory(), c, &mut app);
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 2);
            assert_eq!(ly as u64, line);
        }
        {
            // Mode 3 for indefinate time starting at 84
            let mut app = DummyApp {};
            lcd.tick(memory.get_video_memory(), cycles + 84, &mut app);
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 3);
            assert_eq!(ly as u64, line);
        }
        // By 448, mode should be 0, and should remain till end of scanline
        for c in (cycles + 448)..(cycles + 456) {
            let mut app = DummyApp {};
            lcd.tick(memory.get_video_memory(), c, &mut app);
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
            let mut app = DummyApp {};
            lcd.tick(memory.get_video_memory(), c, &mut app);
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 0);
            assert_eq!(ly as u64, line);
        }
        // Mode 1 for remaining cycles
        for c in (cycles + 4)..(cycles + 456) {
            let mut app = DummyApp {};
            lcd.tick(memory.get_video_memory(), c, &mut app);
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
            let mut app = DummyApp {};
            lcd.tick(memory.get_video_memory(), c, &mut app);
            let stat = memory.get_io(io_regs::STAT);
            let ly = memory.get_io(io_regs::LY);
            assert_eq!(stat & 0b11, 1);
            assert_eq!(ly as u64, line);
        }
    }
}

struct BufferApp {
    buffer: [u8; 160 * 144],
}

impl BufferApp {
    fn new() -> BufferApp {
        BufferApp {
            buffer: [0; 160 * 144],
        }
    }
}

impl App for BufferApp {
    fn draw_line(&mut self, line: &[u8], line_index: u8) {
        for (i, v) in line.iter().enumerate() {
            self.buffer[usize::from(line_index) * 160 + i] = *v;
        }
    }

    fn update(&mut self, joypad: &mut JoyPad) -> Command {
        return Command::Stop;
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

    let mut app = BufferApp::new();

    // Run 1 frame
    for cycles in 0..70224 {
        lcd.tick(&mut vmem, cycles, &mut app);
    }

    let test_file = "test_data/bg_checker_pattern.data";
    test_against(test_file, &app.buffer);
    // write_png("test_data/bg_checker_pattern.png", &app.buffer);
    // dump_test_file(test_file, &app.buffer);
}

#[test]
fn bg_checker_pattern_scx() {
    let mut vmem = VideoMemory::test_new();
    vmem.regs.lcdc = 0b1001_0000;
    vmem.regs.bgp = 0b00_01_10_11;

    color_tile(TILE_DATA_2, 0, 0, &mut vmem);
    color_tile(TILE_DATA_2, 1, 3, &mut vmem);

    for i in 0..1024 {
        vmem[TILE_MAP_1 as usize + i] = (((i % 2) + (i / 32)) % 2) as u8;
    }

    let mut lcd = LCD::new();

    let mut app = BufferApp::new();

    // Run 1 frame
    for cycles in 0..70224 {
        vmem.regs.scx = vmem.regs.ly;
        lcd.tick(&mut vmem, cycles, &mut app);
    }

    let test_file = "test_data/bg_checker_pattern_scx.data";
    test_against(test_file, &app.buffer);
    // write_png("test_data/test.png", &app.buffer);
    // dump_test_file(test_file, &app.buffer);
}

#[test]
fn tetris_render() {
    test_vmem_dump("test_data/tetris.vmem_dump", "test_data/tetris.data");
}

#[test]
fn rockybullwinkle_render() {
    test_vmem_dump(
        "test_data/rockybullwinkle.vmem_dump",
        "test_data/rockybullwinkle.data",
    );
}

#[test]
fn supermarioland_render() {
    test_vmem_dump(
        "test_data/supermarioland.vmem_dump",
        "test_data/supermarioland.data",
    );
}

#[test]
fn ultima_render() {
    test_vmem_dump("test_data/ultima.vmem_dump", "test_data/ultima.data");
}

#[test]
fn pokemon_render() {
    test_vmem_dump(
        "test_data/pokemonblue_outside.vmem_dump",
        "test_data/pokemonblue_outside.data",
    );
    test_vmem_dump(
        "test_data/pokemonblue_fight.vmem_dump",
        "test_data/pokemonblue_fight.data",
    );
}

fn test_vmem_dump(vmem_dump_path: &str, test_data_path: &str) {
    let mut mem = {
        let cart = Cartridge::create_dummy();
        Memory::new(Vec::new(), cart)
    };
    let file = File::open(vmem_dump_path).unwrap();
    let f = BufReader::new(file);
    for line in f.lines() {
        let line = line.unwrap();
        let addr: Vec<&str> = line.split(": ").take(1).collect();
        let addr = u16::from_str_radix(addr[0], 16).unwrap();
        let data: Vec<&str> = line.split(" ").skip(1).collect();
        for (i, v) in data.iter().enumerate() {
            let index = addr + i as u16;
            let v = u8::from_str_radix(v, 16).unwrap();
            mem.set_u8(index, v);
        }
    }
    let mut vmem = mem.get_video_memory();

    let mut lcd = LCD::new();

    let mut app = BufferApp::new();

    // Run 1 frame
    for cycles in 0..70224 {
        lcd.tick(&mut vmem, cycles, &mut app);
    }

    // let png_path = format!("{}.png", test_data_path);
    // write_png(&png_path, &app.buffer);
    // dump_test_file(test_data_path, &app.buffer);
    test_against(test_data_path, &app.buffer);
}

fn color_tile(data_start: u16, index: usize, value: u8, vmem: &mut VideoMemory) {
    assert!(value < 4);
    let data_start = data_start as usize + index * 16;
    for i in 0..8 {
        vmem[data_start + i * 2] = if value & 0b01 != 0 { 0xff } else { 0 };

        vmem[data_start + i * 2 + 1] = if value & 0b10 != 0 { 0xff } else { 0 };
    }
}

#[allow(dead_code)]
fn write_png(fname: &str, buffer: &[u8; 160 * 144]) {
    let mut rgba_buffer = [0; 160 * 144 * 4];

    for (i, v) in buffer.iter().enumerate() {
        let v = *v * 85;

        // Flip coordinates vertically
        let x = i - ((i / 160) * 160);
        let y = 143 - i / 160;
        let i = y * 160 + x;

        rgba_buffer[i * 4] = v;
        rgba_buffer[i * 4 + 1] = v;
        rgba_buffer[i * 4 + 2] = v;
        rgba_buffer[i * 4 + 3] = 255;
    }

    let mut file = File::create(fname).unwrap();
    write_rgba_from_u8(&mut file, &rgba_buffer, 160, 144).unwrap();
    println!("saved image {}", fname);
}

#[allow(dead_code)]
fn dump_test_file(fname: &str, buffer: &[u8; 160 * 144]) {
    let mut file = File::create(fname).unwrap();
    file.write_all(buffer).unwrap();
    println!("saved image {}", fname);
    panic!("DUMPING TEST FILE");
}

fn get_test_data(fname: &str) -> [u8; 160 * 144] {
    let mut file = File::open(fname).unwrap();
    let mut buffer = [0; 160 * 144];
    let length = file.read(&mut buffer).unwrap();
    assert_eq!(length, 160 * 144);
    buffer
}

fn test_against(fname: &str, buffer: &[u8; 160 * 144]) {
    let test_data = get_test_data(fname);
    for (a, b) in buffer.iter().zip(test_data.iter()) {
        assert_eq!(*a, *b);
    }
}
