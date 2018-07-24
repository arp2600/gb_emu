mod mode_updater;
mod pixel_iterator;
mod renderer;
use self::mode_updater::ModeUpdater;
use self::renderer::Renderer;
use super::App;
use memory::VideoMemory;

pub struct LCD {
    update_time: u64,
    enabled: bool,
    frame: u64,
    next_ly: u8,
    vblank_flag: bool,
    mode_updater: ModeUpdater,
    renderer: Renderer,
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
            renderer: Renderer::new(),
        }
    }

    pub fn is_vblank(&self) -> bool {
        self.vblank_flag
    }

    pub fn reset_vblank(&mut self) {
        self.vblank_flag = false;
    }

    pub fn tick<T: App>(&mut self, vram: &mut VideoMemory, cycles: u64, app: &mut T) {
        let enabled = vram.check_enabled();
        if enabled && !self.enabled {
            self.enabled = true;
            self.update_time = cycles;
            self.mode_updater.init(cycles);
            self.next_ly = 0;
            vram.regs.ly = 0;
        } else if !enabled && self.enabled {
            self.enabled = false;
        }

        if self.enabled && cycles >= self.update_time {
            self.update_time += 456;

            let ly = vram.regs.ly;
            if ly == 0 {
                self.renderer.draw_background(vram);
            }

            if ly < 144 {
                self.renderer.draw_line(vram, app);
            } else if ly == 144 {
                self.vblank_flag = true;
            }

            if self.next_ly == 144 {
                self.frame += 1;
                vram.regs.vblank_interrupt_enabled = true;
            }

            let lyc = vram.regs.lyc;

            vram.set_coincidence_flag(ly == lyc);
            if vram.is_lyc_check_enabled() {
                vram.regs.stat_interrupt_enabled = ly == lyc;
            }

            vram.regs.ly = self.next_ly;
            self.next_ly = self.next_ly.wrapping_add(1) % 154;
        }

        if self.enabled {
            self.mode_updater.update(vram, cycles);
        }
    }
}

#[cfg(test)]
mod test;
