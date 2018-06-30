use bit_ops::BitGetSet;
use memory::Memory;
use memory_values::*;

struct PixelIterator {
    i: u8,
    low: u8,
    high: u8,
}

impl PixelIterator {
    fn new(value: u16) -> PixelIterator {
        let low = value as u8;
        let high = (value >> 8) as u8;
        PixelIterator { i: 0, low, high }
    }
}

impl Iterator for PixelIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 8 {
            let low = (self.low >> 7) & 0b1;
            let high = (self.high >> 6) & 0b10;
            self.low <<= 1;
            self.high <<= 1;
            self.i += 1;
            Some((low | high) as u8)
        } else {
            None
        }
    }
}

struct LCDRegisters<'a> {
    memory: &'a mut Memory,
    lcdc: Option<u8>,
    ly: Option<u8>,
    lyc: Option<u8>,
    stat: Option<u8>,
    scy: Option<u8>,
    bgp: Option<u8>,
}

macro_rules! create_getter {
    ( $name:ident, $member:ident, $location:expr ) => {
        pub fn $name(&mut self) -> u8 {
            match self.$member {
                Some(x) => x,
                None => {
                    let x = self.memory.get_io($location);
                    self.$member = Some(x);
                    x
                }
            }
        }
    };
}

macro_rules! create_setter {
    ( $name:ident, $member:ident, $location:expr ) => {
        pub fn $name(&mut self, value: u8) {
            self.$member = Some(value);
            self.memory.set_io($location, value);
        }
    };
}

impl<'a> LCDRegisters<'a> {
    fn new(memory: &mut Memory) -> LCDRegisters {
        LCDRegisters {
            memory,
            lcdc: None,
            ly: None,
            lyc: None,
            stat: None,
            scy: None,
            bgp: None,
        }
    }

    create_getter!(get_lcdc, lcdc, IoRegs::LCDC);

    create_getter!(get_ly, ly, IoRegs::LY);
    create_setter!(set_ly, ly, IoRegs::LY);

    create_getter!(get_lyc, lyc, IoRegs::LYC);

    create_getter!(get_scy, scy, IoRegs::SCY);

    create_getter!(get_bgp, bgp, IoRegs::BGP);

    create_getter!(get_stat, stat, IoRegs::STAT);
    create_setter!(set_stat, stat, IoRegs::STAT);

    pub fn check_enabled(&mut self) -> bool {
        self.get_lcdc().get_bit(7)
    }

    pub fn get_bg_tilemap_display_select(&mut self) -> u16 {
        if self.get_lcdc().get_bit(3) {
            TILE_MAP_2
        } else {
            TILE_MAP_1
        }
    }

    pub fn get_tile_data_select(&mut self) -> u16 {
        if self.get_lcdc().get_bit(4) {
            TILE_DATA_2
        } else {
            TILE_DATA_1
        }
    }

    pub fn set_interrupt_bit(&mut self) {
        let if_reg = self.memory.get_io(IoRegs::IF).set_bit(0);
        self.memory.set_io(IoRegs::IF, if_reg);
    }

    pub fn set_lcd_mode(&mut self, mode: u8) {
        let stat = self.get_stat();
        self.set_stat(stat & 0b1111_1100 | mode & 0b11);
    }

    pub fn set_coincidence_flag(&mut self, state: bool) {
        let stat = self.get_stat();
        if state {
            self.set_stat(stat.set_bit(2));
        } else {
            self.set_stat(stat.reset_bit(2));
        }
    }
}

pub struct LCD {
    update_time: u64,
    enabled: bool,
    frame: u64,
    next_ly: u8,
    vblank_flag: bool,
    mode_state: u8,
    mode_update_time: u64,
}

impl LCD {
    pub fn new() -> LCD {
        LCD {
            update_time: 0,
            enabled: false,
            frame: 0,
            next_ly: 0,
            vblank_flag: false,
            mode_state: 0,
            mode_update_time: 0,
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
            self.mode_update_time = cycles;
            self.mode_state = 0;
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

        if self.enabled && cycles >= self.mode_update_time {
            self.update_mode(&mut regs);
        }
    }

    fn update_mode(&mut self, regs: &mut LCDRegisters) {
        // Ad-hoc state machine
        match self.mode_state {
            0 => {
                regs.set_lcd_mode(0);
                self.mode_update_time += 4;
                let ly = regs.get_ly();
                if ly == 144 {
                    self.mode_state = 4;
                } else {
                    self.mode_state = 1;
                }
            }
            1 => {
                regs.set_lcd_mode(2);
                self.mode_state = 2;
                self.mode_update_time += 80;
            }
            2 => {
                regs.set_lcd_mode(3);
                self.mode_state = 3;
                // About 41 micro seconds
                // from pandocs
                self.mode_update_time += 172;
            }
            3 => {
                regs.set_lcd_mode(0);
                self.mode_update_time += 200;
                self.mode_state = 0;
            }
            4 => {
                regs.set_lcd_mode(1);
                self.mode_update_time += 4556;
                self.mode_state = 0;
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use cartridge::Cartridge;
    use lcd::LCD;
    use memory::Memory;
    use memory_values::IoRegs;

    // Check lcd against old algorithm for calculating ly register
    #[test]
    fn ly_timing() {
        let mut memory = {
            let boot_rom = Vec::new();
            let cartridge = Cartridge::create_dummy();
            Memory::new(boot_rom, cartridge)
        };

        let mut lcd = LCD::new();

        memory.set_io(IoRegs::LCDC, 0b1000_0000);
        // Run for 10 frames
        for cycles in 0..(70224 * 10) {
            let frame_time = cycles % (456 * 154);
            let test_ly = (frame_time / 456) as u8;

            lcd.tick(&mut memory, cycles, |_, _| {});
            let lcd_ly = memory.get_io(IoRegs::LY);

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

        memory.set_io(IoRegs::LCDC, 0b1000_0000);

        {
            let stat = memory.get_io(IoRegs::STAT);
            let ly = memory.get_io(IoRegs::LY);
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
                let stat = memory.get_io(IoRegs::STAT);
                let ly = memory.get_io(IoRegs::LY);
                assert_eq!(stat & 0b11, 0);
                assert_eq!(ly as u64, line);
            }
            // Test line 0 timings
            for c in (cycles + 4)..(cycles + 84) {
                lcd.tick(memory, c, |_, _| {});
                let stat = memory.get_io(IoRegs::STAT);
                let ly = memory.get_io(IoRegs::LY);
                assert_eq!(stat & 0b11, 2);
                assert_eq!(ly as u64, line);
            }
            {
                // Mode 3 for indefinate time starting at 84
                lcd.tick(memory, cycles + 84, |_, _| {});
                let stat = memory.get_io(IoRegs::STAT);
                let ly = memory.get_io(IoRegs::LY);
                assert_eq!(stat & 0b11, 3);
                assert_eq!(ly as u64, line);
            }
            // By 448, mode should be 0, and should remain till end of scanline
            for c in (cycles + 448)..(cycles + 456) {
                lcd.tick(memory, c, |_, _| {});
                let stat = memory.get_io(IoRegs::STAT);
                let ly = memory.get_io(IoRegs::LY);
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
                let stat = memory.get_io(IoRegs::STAT);
                let ly = memory.get_io(IoRegs::LY);
                assert_eq!(stat & 0b11, 0);
                assert_eq!(ly as u64, line);
            }
            // Mode 1 for remaining cycles
            for c in (cycles + 4)..(cycles + 456) {
                lcd.tick(memory, c, |_, _| {});
                let stat = memory.get_io(IoRegs::STAT);
                let ly = memory.get_io(IoRegs::LY);
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
                let stat = memory.get_io(IoRegs::STAT);
                let ly = memory.get_io(IoRegs::LY);
                assert_eq!(stat & 0b11, 1);
                assert_eq!(ly as u64, line);
            }
        }
    }
}
