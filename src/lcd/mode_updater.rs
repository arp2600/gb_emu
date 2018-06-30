use super::lcd_registers::LCDRegisters;

#[derive(Default)]
pub struct ModeUpdater {
    state: u8,
    update_time: u64,
}

impl ModeUpdater {
    pub fn init(&mut self, update_time: u64) {
        self.update_time = update_time;
        self.state = 0;
    }

    pub fn update(&mut self, regs: &mut LCDRegisters, cycles: u64) {
        if cycles >= self.update_time {
            self.update_mode(regs);
        }
    }

    fn update_mode(&mut self, regs: &mut LCDRegisters) {
        // Ad-hoc state machine
        match self.state {
            0 => {
                regs.set_lcd_mode(0);
                self.update_time += 4;
                let ly = regs.get_ly();
                if ly == 144 {
                    self.state = 4;
                } else {
                    self.state = 1;
                }
            }
            1 => {
                regs.set_lcd_mode(2);
                self.state = 2;
                self.update_time += 80;
            }
            2 => {
                regs.set_lcd_mode(3);
                self.state = 3;
                // About 41 micro seconds
                // from pandocs
                self.update_time += 172;
            }
            3 => {
                regs.set_lcd_mode(0);
                self.update_time += 200;
                self.state = 0;
            }
            4 => {
                regs.set_lcd_mode(1);
                self.update_time += 4556;
                self.state = 0;
            }
            _ => unreachable!(),
        }
    }
}
