use super::bit_ops::BitGetSet;
use super::cpu;
use super::memory::{io_regs, Memory};

pub struct Timer {
    enabled: bool,
    input_clock: u64,
    update_time: u64,
    div_update_time: u64,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            enabled: false,
            input_clock: 4096,
            update_time: 0,
            div_update_time: 0,
        }
    }

    pub fn tick(&mut self, memory: &mut Memory, cycles: u64) {
        let (tima, tma) = self.read_registers(memory, cycles);
        if self.enabled && cycles > self.update_time {
            self.update_time += self.cpu_cycles_per_tick();

            if tima == 255 {
                memory.set_io(io_regs::TIMA, tma);
                // println!("TIMA set to TMA({})", tma);
                let if_reg = memory.get_io(io_regs::IF).set_bit(2);
                memory.set_io(io_regs::IF, if_reg);
            } else {
                // println!("TIMA set to {}", tima + 1);
                memory.set_io(io_regs::TIMA, tima + 1);
            }
        }

        if cycles > self.div_update_time {
            let div = memory.get_io(io_regs::DIV).wrapping_add(1);
            memory.set_io(io_regs::DIV, div);

            self.div_update_time += self.cpu_cycles_per_div_increment();
        }
    }

    fn cpu_cycles_per_tick(&mut self) -> u64 {
        cpu::CLOCK_SPEED / self.input_clock
    }

    fn cpu_cycles_per_div_increment(&mut self) -> u64 {
        cpu::CLOCK_SPEED / 16_384
    }

    fn read_registers(&mut self, memory: &mut Memory, cycles: u64) -> (u8, u8) {
        let tima = memory.get_io(io_regs::TIMA);
        let tma = memory.get_io(io_regs::TMA);
        let tac = memory.get_io(io_regs::TAC);

        let enabled = tac.get_bit(2);
        let input_clock = match tac & 0b11 {
            0b00 => 4_096,
            0b01 => 262_144,
            0b10 => 65_536,
            0b11 => 16_384,
            _ => unreachable!(),
        };

        if enabled != self.enabled || input_clock != self.input_clock {
            self.enabled = enabled;
            self.input_clock = input_clock;
            // cpu_cycles_per_tick should be called after updating input clock
            self.update_time = cycles + self.cpu_cycles_per_tick();
            // println!("timer.enabled = {}", self.enabled);
        }

        (tima, tma)
    }
}
