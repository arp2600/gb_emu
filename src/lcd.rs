use bit_ops::BitGetSet;
use memory::Memory;
use memory_values::*;

pub trait Screen {
    fn write_line(&mut self, ly: u8, pixels: &[u8; 160]);
    fn end_frame(&mut self);
}

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
}

pub struct LCD {
    update_time: u64,
    enabled: bool,
    frame: u64,
    next_ly: u8,
}

impl LCD {
    pub fn new() -> LCD {
        LCD {
            update_time: 0,
            enabled: false,
            frame: 0,
            next_ly: 0,
        }
    }

    fn draw_line(&mut self, regs: &mut LCDRegisters) -> [u8; 160] {
        let ly = regs.get_ly();
        let mut line = [0; 160];
        let bgp = {
            let x = regs.get_bgp();
            [3 - (x & 0b11),
             3 - ((x >> 2) & 0b11),
             3 - ((x >> 4) & 0b11),
             3 - ((x >> 6) & 0b11)]
        };
        // Look at each tile on the current line
        for x in 0..(160/8) {
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
        line
    }

    pub fn tick(&mut self, memory: &mut Memory, cycles: u64, screen: Option<&mut Screen>) {
        let mut regs = LCDRegisters::new(memory);
        let enabled = regs.check_enabled();
        if enabled && !self.enabled {
            self.enabled = true;
            self.update_time = cycles;
            self.next_ly = 0;
            regs.set_ly(0);
        }

        if self.enabled && cycles >= self.update_time {
            self.update_time += 456;

            let ly = regs.get_ly();
            if let Some(s) = screen {
                if ly <= 144 {
                    let line = self.draw_line(&mut regs);
                    s.write_line(ly, &line);
                }
            }

            if self.next_ly == 144 {
                self.frame += 1;
                regs.set_interrupt_bit();
            }

            let lyc = regs.get_lyc();

            regs.set_stat(if ly == lyc { 0b10 } else { 0 });

            regs.set_ly(self.next_ly);
            self.next_ly = self.next_ly.wrapping_add(1) % 154;
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
    fn lcd_test() {
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

            lcd.tick(&mut memory, cycles, None);
            let lcd_ly = memory.get_io(IoRegs::LY);

            assert_eq!(
                lcd_ly, test_ly,
                "lcd {} != test {} at cycles {}",
                lcd_ly, test_ly, cycles
            );
        }
    }
}
