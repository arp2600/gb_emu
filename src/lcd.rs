use bit_ops::BitGetSet;
use memory::Memory;
use memory_values::*;

pub trait Screen {
    fn write_line(&mut self, ly: u8, pixels: &[u8; 256]);
    fn end_frame(&mut self);
}

struct LCDRegisters<'a> {
    memory: &'a mut Memory,
    lcdc: Option<u8>,
    ly: Option<u8>,
    lyc: Option<u8>,
    stat: Option<u8>,
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
        }
    }

    create_getter!(get_lcdc, lcdc, IoRegs::LCDC);

    create_getter!(get_ly, ly, IoRegs::LY);
    create_setter!(set_ly, ly, IoRegs::LY);

    create_getter!(get_lyc, ly, IoRegs::LYC);
    create_setter!(set_lyc, ly, IoRegs::LYC);

    create_setter!(set_stat, stat, IoRegs::STAT);

    pub fn is_enabled(&mut self) -> bool {
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

    fn draw_line(&mut self, regs: &mut LCDRegisters) -> [u8; 256] {
        let ly = regs.get_ly();
        let mut line = [0; 256];
        // Look at each tile on the current line
        for x in 0..32 {
            let y = u16::from(ly / 8);

            // Get the index of the tile
            let tile_map = regs.get_bg_tilemap_display_select();
            let index = tile_map + x + 32 * y;
            let tile_index = regs.memory.get_u8(index) as u16;

            // Get the address of the tile
            let tile_data = regs.get_tile_data_select();
            let tile_address = tile_data + tile_index * 16;
            let tile_y_index = u16::from(ly % 8);
            let line_address = tile_address + tile_y_index * 2;

            let pixels = {
                let t = regs.memory.get_u16(line_address);
                t.reverse_bits()
            };

            // let tile_index = regs.get_bg_tile_map(x, y);
            for i in 0..8 {
                let pixel = (pixels >> i*2) & 0b11;
                let line_index = usize::from(x * 8 + i);
                // line[line_index] = tile_index as u8;
                line[line_index] = pixel as u8;
            }
        }
        line
    }

    pub fn tick(&mut self, memory: &mut Memory, cycles: u64, screen: Option<&mut Screen>) {
        let mut regs = LCDRegisters::new(memory);
        let enabled = regs.is_enabled();
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

            if ly == 144 {
                self.frame += 1;
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
