use super::bit_ops::BitGetSet;
use super::cpu;
use super::memory::Memory;
use super::memory_values::IoRegs;

pub struct Timer {
    enabled: bool,
    input_clock: u64,
    update_time: u64,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            enabled: false,
            input_clock: 4096,
            update_time: 0,
        }
    }

    pub fn tick(&mut self, memory: &mut Memory, cycles: u64) {
        let (tima, tma) = self.read_registers(memory, cycles);
        if self.enabled {
            if cycles > self.update_time {
                self.update_time += self.cpu_cycles_per_tick();

                if tima == 255 {
                    memory.set_u8(IoRegs::TIMA as u16, tma);
                    // println!("TIMA set to TMA({})", tma);
                    let if_reg = memory.get_u8(IoRegs::IF as u16).set_bit(2);
                    memory.set_u8(IoRegs::IF as u16, if_reg);
                } else {
                    // println!("TIMA set to {}", tima + 1);
                    memory.set_u8(IoRegs::TIMA as u16, tima + 1);
                }
            }
        }
    }

    fn cpu_cycles_per_tick(&mut self) -> u64 {
        cpu::CLOCK_SPEED / self.input_clock
    }

    fn read_registers(&mut self, memory: &mut Memory, cycles: u64) -> (u8, u8) {
        let tima = memory.get_u8(IoRegs::TIMA as u16);
        let tma = memory.get_u8(IoRegs::TMA as u16);
        let tac = memory.get_u8(IoRegs::TAC as u16);

        let enabled = tac.get_bit(2);
        let input_clock = match tac & 0b11 {
            0b00 => 4096,
            0b01 => 262144,
            0b10 => 65536,
            0b11 => 16384,
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
