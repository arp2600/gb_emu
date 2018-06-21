use memory::Memory;
use memory_values::IoRegs;

pub trait Screen {
    fn write_line(&mut self, ly: u8, pixels: &[u8; 160]);
    fn end_frame(&mut self);
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

    pub fn tick(&mut self, memory: &mut Memory, cycles: u64, screen: Option<&mut Screen>) {
        let enabled = memory.get_io(IoRegs::LCDC) & 0b1000_0000 != 0;
        if enabled && !self.enabled {
            self.enabled = true;
            self.update_time = cycles;
            self.next_ly = 0;
            memory.set_io(IoRegs::LY, 0);
        }

        if self.enabled && cycles >= self.update_time {
            self.update_time += 456;

            let ly = memory.get_io(IoRegs::LY);
            if let Some(s) = screen {
                if ly <= 144 {
                    let mut line = [0; 160];
                    for i in 0..160 {
                        let x = (ly.wrapping_add(i)) % 2;
                        line[i as usize] = x;
                    }
                    s.write_line(ly, &line);
                }
            }

            if ly == 144 {
                self.frame += 1;
            }

            let lyc = memory.get_io(IoRegs::LYC);

            if ly == lyc {
                memory.set_io(IoRegs::STAT, 0b10);
            }

            memory.set_io(IoRegs::LY, self.next_ly);
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
