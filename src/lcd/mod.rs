mod lcd_registers;
mod mode_updater;
mod pixel_iterator;
use self::lcd_registers::LCDRegisters;
use self::mode_updater::ModeUpdater;
use self::pixel_iterator::PixelIterator;
use memory::Memory;

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

    fn draw_line<F>(&mut self, regs: &mut LCDRegisters, mut draw_fn: F)
    where
        F: FnMut(&[u8], u8),
    {
        let ly = regs.get_ly();
        let mut line = [0; 160];

        let bgp = {
            let x = regs.get_bgp();
            [
                3 - (x & 0b11),
                3 - ((x >> 2) & 0b11),
                3 - ((x >> 4) & 0b11),
                3 - ((x >> 6) & 0b11),
            ]
        };

        // Look at each tile on the current line
        for x in 0..(160 / 8) {
            let scy = regs.get_scy();
            let ly_scy = ly.wrapping_add(scy);
            let y = u16::from(ly_scy / 8);

            // Get the index of the tile data
            let tile_map = regs.get_bg_tilemap_display_select();
            let tile_data_index = {
                let i = tile_map + x + 32 * y;
                u16::from(regs.memory.get_u8(i))
            };

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

        draw_fn(&line, ly);
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
                self.draw_line(&mut regs, &mut draw_fn);
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
