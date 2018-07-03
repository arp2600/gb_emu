mod lcd_registers;
mod mode_updater;
mod pixel_iterator;
use self::lcd_registers::{LCDRegisters, DrawData};
use self::mode_updater::ModeUpdater;
use self::pixel_iterator::PixelIterator;
use memory::Memory;
use memory_values::*;
use super::bit_ops::BitGetSet;

pub struct LCD {
    update_time: u64,
    enabled: bool,
    frame: u64,
    next_ly: u8,
    vblank_flag: bool,
    mode_updater: ModeUpdater,
}

impl LCD {
    pub fn new() -> LCD {
        LCD {
            update_time: 0,
            enabled: false,
            frame: 0,
            next_ly: 0,
            vblank_flag: false,
            mode_updater: Default::default(),
        }
    }

    pub fn is_vblank(&self) -> bool {
        self.vblank_flag
    }

    pub fn reset_vblank(&mut self) {
        self.vblank_flag = false;
    }

    pub fn tick<F>(&mut self, memory: &mut Memory, cycles: u64, mut draw_fn: F)
    where
        F: FnMut(&[u8], u8),
    {
        let mut regs = LCDRegisters::new(memory);
        let enabled = regs.check_enabled();
        if enabled && !self.enabled {
            self.enabled = true;
            self.update_time = cycles;
            self.mode_updater.init(cycles);
            self.next_ly = 0;
            regs.set_ly(0);
        }

        if self.enabled && cycles >= self.update_time {
            self.update_time += 456;

            let ly = regs.get_ly();
            if ly < 144 {
                let draw_data = DrawData::new(&mut regs);
                draw_line(&draw_data, &mut draw_fn);
            } else if ly == 144 {
                self.vblank_flag = true;
            }

            if self.next_ly == 144 {
                self.frame += 1;
                regs.set_interrupt_bit();
            }

            let lyc = regs.get_lyc();

            regs.set_coincidence_flag(ly == lyc);

            regs.set_ly(self.next_ly);
            self.next_ly = self.next_ly.wrapping_add(1) % 154;
        }

        if self.enabled {
            self.mode_updater.update(&mut regs, cycles);
        }
    }
}

fn create_bgp_data(bgp_value: u8) -> [u8; 4] {
    [
        3 - (bgp_value & 0b11),
        3 - ((bgp_value >> 2) & 0b11),
        3 - ((bgp_value >> 4) & 0b11),
        3 - ((bgp_value >> 6) & 0b11),
    ]
}

fn get_bg_tile_index (x: u16, y: u16, regs: &DrawData) -> u16 {
    let tile_map = regs.get_bg_tilemap_display_select();
    let i = tile_map + x + 32 * y;
    u16::from(regs.memory.get_u8(i))
}

fn draw_bg(regs: &DrawData, line: &mut[u8; 160]) {
    let ly = regs.ly;
    let bgp = create_bgp_data(regs.bgp);

    // Look at each tile on the current line
    for x in 0..(160 / 8) {
        let scy = regs.scy;
        let ly_scy = ly.wrapping_add(scy);
        let y = u16::from(ly_scy / 8);

        // Get the index of the tile data
        let tile_data_index = get_bg_tile_index(x, y, regs);

        // Get the address of the tile
        let tile_data_start = regs.get_tile_data_select();
        let tile_address = tile_data_start + tile_data_index * 16;
        let tile_y_index = u16::from(ly_scy % 8);
        let line_address = tile_address + tile_y_index * 2;

        let pixels = regs.memory.get_u16(line_address);
        for (i, pixel) in PixelIterator::new(pixels).enumerate() {
            let line_index = usize::from(x * 8) + i;
            line[line_index] = bgp[pixel as usize];
        }
    }
}

fn draw_sprites(regs: &DrawData, line: &mut[u8; 160]) {
    if !regs.are_sprites_enabled() {
        return;
    }

    for i in 0..40 {
        let oam_index = SPRITE_ATTRIBUTE_TABLE + i * 4;
        let y = regs.memory.get_u8(oam_index).wrapping_sub(9);
        let x = regs.memory.get_u8(oam_index + 1).wrapping_sub(8);
        if y >= regs.ly && y < (regs.ly + 8) {
            let tile_num = regs.memory.get_u8(oam_index + 2) as u16;
            let attributes = regs.memory.get_u8(oam_index + 3);
            let y_flip = attributes.get_bit(6);
            let palette = attributes.get_bit(4) as u8;
            let obp = create_bgp_data(regs.get_obp(palette));
            let tile_address = SPRITE_PATTERN_TABLE + tile_num * 16;
            let tile_y_index = {
                let y = u16::from(y - regs.ly);
                if y_flip {
                    y
                } else {
                    7 - y
                }
            };
            let line_address = tile_address + tile_y_index * 2;

            let pixels = regs.memory.get_u16(line_address);
            for (i, pixel) in PixelIterator::new(pixels).enumerate() {
                let index = usize::from(x) + i;
                if index < line.len() {
                    if pixel > 0 {
                        line[index] = obp[pixel as usize];
                    }
                }
            }
        }
    }
}

fn draw_line<F>(regs: &DrawData, mut draw_fn: F)
where
    F: FnMut(&[u8], u8),
{
    let mut line = [0; 160];

    draw_bg(regs, &mut line);
    draw_sprites(regs, &mut line);

    let ly = regs.ly;
    draw_fn(&line, ly);
}

#[cfg(test)]
mod tests {
    use cartridge::Cartridge;
    use lcd::LCD;
    use memory::Memory;
    use memory_values::io_regs;

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

            lcd.tick(&mut memory, cycles, |_, _| {});
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
                lcd.tick(memory, c, |_, _| {});
                let stat = memory.get_io(io_regs::STAT);
                let ly = memory.get_io(io_regs::LY);
                assert_eq!(stat & 0b11, 0);
                assert_eq!(ly as u64, line);
            }
            // Test line 0 timings
            for c in (cycles + 4)..(cycles + 84) {
                lcd.tick(memory, c, |_, _| {});
                let stat = memory.get_io(io_regs::STAT);
                let ly = memory.get_io(io_regs::LY);
                assert_eq!(stat & 0b11, 2);
                assert_eq!(ly as u64, line);
            }
            {
                // Mode 3 for indefinate time starting at 84
                lcd.tick(memory, cycles + 84, |_, _| {});
                let stat = memory.get_io(io_regs::STAT);
                let ly = memory.get_io(io_regs::LY);
                assert_eq!(stat & 0b11, 3);
                assert_eq!(ly as u64, line);
            }
            // By 448, mode should be 0, and should remain till end of scanline
            for c in (cycles + 448)..(cycles + 456) {
                lcd.tick(memory, c, |_, _| {});
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
                lcd.tick(memory, c, |_, _| {});
                let stat = memory.get_io(io_regs::STAT);
                let ly = memory.get_io(io_regs::LY);
                assert_eq!(stat & 0b11, 0);
                assert_eq!(ly as u64, line);
            }
            // Mode 1 for remaining cycles
            for c in (cycles + 4)..(cycles + 456) {
                lcd.tick(memory, c, |_, _| {});
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
                lcd.tick(memory, c, |_, _| {});
                let stat = memory.get_io(io_regs::STAT);
                let ly = memory.get_io(io_regs::LY);
                assert_eq!(stat & 0b11, 1);
                assert_eq!(ly as u64, line);
            }
        }
    }
}
