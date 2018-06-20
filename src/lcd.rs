use memory::Memory;
use memory_values::IoRegs;

pub struct LCD {
    update_time: u64,
    enabled: bool,
}

impl LCD {
    pub fn new() -> LCD {
        LCD {
            update_time: 0,
            enabled: false,
        }
    }

    pub fn tick(&mut self, memory: &mut Memory, cycles: u64) {
        let enabled = memory.get_io(IoRegs::LCDC) & 0b1000_0000 != 0;
        if enabled && !self.enabled {
            self.enabled = true;
            // 455 means this is the first cycle
            // this is to pass testing against old lcd implementation
            self.update_time = cycles + 455;
            memory.set_io(IoRegs::LY, 0);
        }

        if self.enabled && cycles > self.update_time {
            self.update_time += 456;

            let ly = (memory.get_io(IoRegs::LY) + 1) % 154;
            let lyc = memory.get_io(IoRegs::LYC);

            if ly == lyc {
                memory.set_io(IoRegs::STAT, 0b10);
            }

            memory.set_io(IoRegs::LY, ly);
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

            lcd.tick(&mut memory, cycles);
            let lcd_ly = memory.get_io(IoRegs::LY);

            assert_eq!(lcd_ly, test_ly);
        }
    }
}
