use super::memory::Memory;
use super::registers::Registers;

pub struct Cpu<'a> {
    registers: &'a mut Registers,
    instruction_counter: usize,
    cycles: u64,
}

impl<'a> Cpu<'a> {
    pub fn new(registers: &'a mut Registers) -> Cpu<'a> {
        Cpu {
            registers,
            instruction_counter: 0,
            cycles: 0,
        }
    }

    pub fn get_cycles(&mut self) -> u64 {
        self.cycles
    }

    pub fn tick(&mut self, memory: &mut Memory) {
        self.instruction_counter += 1;
        let mnemonic = self.get_opcode_mnemonic(memory);
        self.fetch_and_execute(memory);
        println!("{} {:?} {}", mnemonic, self.registers, self.cycles);
    }

    fn get_opcode_mnemonic(&self, memory: &Memory) -> &'static str {
        let opcode = memory.get_u8(self.registers.pc);

        match opcode {
            0xcb => self.get_cb_opcode_mnemonic(memory),
            0x01 | 0x11 | 0x21 | 0x31 => "LD",
            0xab...0xaf => "XOR",
            0x32 => "LDD",
            0x20 | 0x28 | 0x30 | 0x38 => "JR",
            0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e => "LD",
            0x0a | 0x1a | 0x3e | 0x78...0x7f | 0xfa => "LD",
            0xe2 => "LDH",
            0x3c | 0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x34 => {
                "INC"
            }
            0x47 | 0x4F | 0x57 | 0x5F | 0x67 | 0x6F | 0x02 | 0x12 | 0x77 | 0xEA => {
                "LD"
            }
            0xe0 => "LDH",
            0xcd => "CALL",
            0xf5 | 0xc5 | 0xd5 | 0xe5 => "PUSH",
            0xf1 | 0xc1 | 0xd1 | 0xe1 => "POP",
            0x17 => "RL",
            0x3d | 0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x35 => {
                "DEC"
            }
            0x22 => "LDI",
            0x03 | 0x13 | 0x23 | 0x33 => "INC",
            0xc9 => "RET",
            0xb8...0xbf | 0xfe => "CP",
            0x18 => "JR",
            0xf0 => "LDH",
            0x90...0x97 | 0xd6 => "SUB",
            _ => "__",
        }
    }

    fn get_cb_opcode_mnemonic(&self, memory: &Memory) -> &'static str {
        let opcode = memory.get_u8(self.registers.pc + 1);

        match opcode {
            0x40...0x7f => "BIT",
            0x10...0x17 => "RL",
            _ => "CB__",
        }
    }

    fn fetch_and_execute(&mut self, memory: &mut Memory) {
        let opcode = memory.get_u8(self.registers.pc);

        match opcode {
            0xcb => self.fetch_and_execute_cb(memory),
            0x01 | 0x11 | 0x21 | 0x31 => self.ld_n_nn(opcode, memory),
            0xab...0xaf => self.xor(opcode, memory),
            0x32 => self.ldd_hl_a(memory),
            0x20 | 0x28 | 0x30 | 0x38 => self.jr_cc_n(opcode, memory),
            0x06 | 0x0e | 0x16 | 0x1e | 0x26 | 0x2e => {
                self.ld_nn_n(opcode, memory);
            }
            0x0a | 0x1a | 0x3e | 0x78...0x7f | 0xfa => {
                self.ld_a_n(opcode, memory);
            }
            0xe2 => self.ld_c_a(memory),
            0x3c | 0x04 | 0x0c | 0x14 | 0x1c | 0x24 | 0x2c | 0x34 => {
                self.inc_n(opcode, memory);
            }
            0x47 | 0x4F | 0x57 | 0x5F | 0x67 | 0x6F | 0x02 | 0x12 | 0x77 | 0xEA => {
                self.ld_n_a(opcode, memory);
            }
            0xe0 => self.ldh_n_a(memory),
            0xcd => self.call_nn(memory),
            0xf5 | 0xc5 | 0xd5 | 0xe5 => self.push_nn(opcode, memory),
            0xf1 | 0xc1 | 0xd1 | 0xe1 => self.pop_nn(opcode, memory),
            0x17 => self.rla(),
            0x3d | 0x05 | 0x0d | 0x15 | 0x1d | 0x25 | 0x2d | 0x35 => {
                self.dec_n(opcode, memory);
            }
            0x22 => self.ldi_hl_a(memory),
            0x03 | 0x13 | 0x23 | 0x33 => self.inc_nn(opcode),
            0xc9 => self.ret(memory),
            0xb8...0xbf | 0xfe => self.cp_n(opcode, memory),
            0x18 => self.jr_n(memory),
            0xf0 => self.ldh_a_n(memory),
            0x90...0x97 | 0xd6 => self.sub_n(opcode, memory),
            _ => panic!("Instruction 0x{:02x} not implemented", opcode),
        }
    }

    fn fetch_and_execute_cb(&mut self, memory: &Memory) {
        self.registers.pc += 1;
        let opcode = memory.get_u8(self.registers.pc);

        match opcode {
            0x40...0x7f => self.bit_b_r(opcode, memory),
            0x10...0x17 => self.rl_n(opcode, memory),
            _ => panic!("Instruction 0xcb{:02x} not implemented", opcode),
        }
    }

    fn get_source_u8(&mut self, index: u8, memory: &Memory) -> u8 {
        match index {
            0 => self.registers.b,
            1 => self.registers.c,
            2 => self.registers.d,
            3 => self.registers.e,
            4 => self.registers.h,
            5 => self.registers.l,
            6 => {
                let hl = self.registers.get_hl();
                memory.get_u8(hl)
            }
            7 => self.registers.a,
            _ => panic!("Bad register {}", index),
        }
    }

    fn set_dest_u8(&mut self, index: u8, value: u8) {
        match index {
            0 => self.registers.b = value,
            1 => self.registers.c = value,
            2 => self.registers.d = value,
            3 => self.registers.e = value,
            4 => self.registers.h = value,
            5 => self.registers.l = value,
            // 6 => {
            //     let hl = self.registers.get_hl();
            //     memory.get_u8(hl)
            // }
            7 => self.registers.a = value,
            _ => panic!("Bad register {}", index),
        }
    }

    fn load_imm_u8(&mut self, memory: &Memory) -> u8 {
        memory.get_u8(self.registers.pc + 1)
    }

    fn load_imm_u16(&mut self, memory: &Memory) -> u16 {
        memory.get_u16(self.registers.pc + 1)
    }

    /************************************************************
                         Opcodes
    ************************************************************/

    fn sub_n(&mut self, opcode: u8, memory: &Memory) {
        let n = match opcode {
            0x90...0x97 => {
                let reg_index = opcode & 0b0000_0111;
                self.get_source_u8(reg_index, memory)
            }
            0xd6 => {
                let n = self.load_imm_u8(memory);
                self.registers.pc += 1;
                n
            }
            _ => panic!("Bad opcode {}", opcode),
        };

        let a = self.registers.a;
        self.registers.a = a - n;

        self.registers.set_flagz(a <= n);
        self.registers.set_flagn(true);
        self.registers.set_flagh(a & 0xf < n & 0xf);
        self.registers.set_flagc(a < n);

        self.registers.pc += 1;
        match opcode {
            0x96 | 0xd6 => self.cycles += 8,
            _ => self.cycles += 4,
        }
    }

    fn ldh_a_n(&mut self, memory: &Memory) {
        let n = self.load_imm_u8(memory);
        let v = memory.get_u8(0xff00 + n as u16);
        self.registers.a = v;
        self.registers.pc += 2;
        self.cycles += 12;
    }

    fn jr_n(&mut self, memory: &Memory) {
        let n = self.load_imm_u8(memory);
        self.registers.pc = signed_add_u16_u8(self.registers.pc + 2, n);
        self.cycles += 12;
    }

    fn cp_n(&mut self, opcode: u8, memory: &Memory) {
        let n = match opcode {
            0xb8...0xbf => {
                let reg_index = opcode & 0b0000_0111;
                self.get_source_u8(reg_index, memory)
            }
            0xfe => {
                let n = self.load_imm_u8(memory);
                self.registers.pc += 1;
                n
            }
            _ => panic!("Bad opcode {}", opcode),
        };

        let a = self.registers.a;

        self.registers.set_flagz(a == n);
        self.registers.set_flagn(true);
        self.registers.set_flagh(a & 0xf < n & 0xf);
        self.registers.set_flagc(a < n);

        self.registers.pc += 1;
        match opcode {
            0xfe | 0xbe => self.cycles += 8,
            _ => self.cycles += 4,
        }
    }

    fn ret(&mut self, memory: &Memory) {
        let sp = self.registers.sp;
        let addr = memory.get_u16(sp);
        self.registers.sp += 2;
        self.registers.pc = addr;
        self.cycles += 16;
    }

    fn ldi_hl_a(&mut self, memory: &mut Memory) {
        let hl = self.registers.hli();
        memory.set_u8(hl, self.registers.a);
        self.registers.pc += 1;
        self.cycles += 8;
    }

    fn dec_n(&mut self, opcode: u8, memory: &Memory) {
        let reg_index = (opcode & 0b0011_1000) >> 3;
        let source = self.get_source_u8(reg_index, memory);
        let result = source.wrapping_sub(1);

        self.registers.set_flagz(result == 0);
        self.registers.set_flagn(true);
        self.registers.set_flagh(source & 0xf == 0);

        self.set_dest_u8(reg_index, result);
        self.registers.pc += 1;
        match opcode {
            0x35 => self.cycles += 12,
            _ => self.cycles += 4,
        }
    }

    fn pop_nn(&mut self, opcode: u8, memory: &Memory) {
        let sp = self.registers.sp;
        let value = memory.get_u16(sp);

        match opcode {
            0xf1 => self.registers.set_af(value),
            0xc1 => self.registers.set_bc(value),
            0xd1 => self.registers.set_de(value),
            0xe1 => self.registers.set_hl(value),
            _ => panic!("Bad opcode {}", opcode),
        };

        self.registers.sp += 2;
        self.registers.pc += 1;
        self.cycles += 12;
    }

    fn rla(&mut self) {
        let a = self.registers.a;
        let mut value = a << 1;
        if self.registers.flagc() {
            value += 1;
        };

        self.registers.clear_flags();
        // Setting flag z here doesn't match the behaviour
        // of the reference emulator.
        // self.registers.set_flagz(value == 0);
        self.registers.set_flagc((a & 0b1000_0000) != 0);

        self.registers.a = value;
        self.registers.pc += 1;
        self.cycles += 4;
    }

    fn rl_n(&mut self, opcode: u8, memory: &Memory) {
        let reg_index = opcode & 0b0000_0111;
        let source = self.get_source_u8(reg_index, memory);
        let mut value = source << 1;
        if self.registers.flagc() {
            value += 1;
        };

        self.registers.clear_flags();
        self.registers.set_flagz(value == 0);
        self.registers.set_flagc((source & 0b1000_0000) != 0);

        self.set_dest_u8(reg_index, value);
        self.registers.pc += 1;
        match opcode {
            0x16 => self.cycles += 16,
            _ => self.cycles += 8,
        }
    }

    fn push_nn(&mut self, opcode: u8, memory: &mut Memory) {
        let value = match opcode {
            0xf5 => self.registers.get_af(),
            0xc5 => self.registers.get_bc(),
            0xd5 => self.registers.get_de(),
            0xe5 => self.registers.get_hl(),
            _ => panic!("Bad opcode {}", opcode),
        };

        memory.set_u16(self.registers.sp - 2, value);
        self.registers.sp -= 2;
        self.registers.pc += 1;
        self.cycles += 16;
    }

    fn call_nn(&mut self, memory: &mut Memory) {
        let addr = self.load_imm_u16(memory);
        self.registers.pc += 3;
        self.registers.sp -= 2;
        memory.set_u16(self.registers.sp, self.registers.pc);
        self.registers.pc = addr;
        self.cycles += 24;
    }

    fn ldh_n_a(&mut self, memory: &mut Memory) {
        let addr = self.load_imm_u8(memory) as u16 + 0xff00;
        memory.set_u8(addr, self.registers.a);
        self.registers.pc += 2;
        self.cycles += 12;
    }

    fn ld_n_a(&mut self, opcode: u8, memory: &mut Memory) {
        let value = self.registers.a;
        match opcode {
            0x7f | 0x47 | 0x4f | 0x57 | 0x5f | 0x67 | 0x6f => {
                let reg_index = (opcode & 0b0011_1000) >> 3;
                self.set_dest_u8(reg_index, value);
            }
            0x02 => {
                let addr = self.registers.get_bc();
                memory.set_u8(addr, value);
            }
            0x12 => {
                let addr = self.registers.get_de();
                memory.set_u8(addr, value);
            }
            0x77 => {
                let addr = self.registers.get_hl();
                memory.set_u8(addr, value);
            }
            0xea => {
                let addr = self.load_imm_u16(memory);
                memory.set_u8(addr, value);
                self.registers.pc += 2;
            }
            x => panic!("Bad opcode {}", x),
        };
        self.registers.pc += 1;
        match opcode {
            0xea => self.cycles += 16,
            0x02 | 0x12 | 0x77 => self.cycles += 8,
            _ => self.cycles += 4,
        };
    }

    fn inc_nn(&mut self, opcode: u8) {
        match opcode {
            0x03 => {
                let source = self.registers.get_bc();
                self.registers.set_bc(source + 1);
            }
            0x13 => {
                let source = self.registers.get_de();
                self.registers.set_de(source + 1);
            }
            0x23 => {
                let source = self.registers.get_hl();
                self.registers.set_hl(source + 1);
            }
            0x33 => {
                let source = self.registers.sp;
                self.registers.sp = source + 1;
            }
            _ => panic!("Bad opcode {}", opcode),
        };

        self.registers.pc += 1;
        self.cycles += 8;
    }

    fn inc_n(&mut self, opcode: u8, memory: &Memory) {
        let reg_index = (opcode & 0b11_1000) >> 3;
        let source = self.get_source_u8(reg_index, memory);
        let result = source + 1;
        self.registers.set_flagz(result == 0);
        self.registers.set_flagn(false);
        self.registers.set_flagh((source & 0xf) + 1 > 0xf);
        self.set_dest_u8(reg_index, result);
        self.registers.pc += 1;
        match opcode {
            0x34 => self.cycles += 12,
            _ => self.cycles += 4,
        }
    }

    fn ld_a_n(&mut self, opcode: u8, memory: &Memory) {
        let n = match opcode {
            0x78...0x7f => self.get_source_u8(opcode & 0b111, memory),
            0x3e => self.load_imm_u8(memory),
            0xfa => {
                let v = self.load_imm_u16(memory);
                memory.get_u8(v)
            }
            0x0a => {
                let bc = self.registers.get_bc();
                memory.get_u8(bc)
            }
            0x1a => {
                let de = self.registers.get_de();
                memory.get_u8(de)
            }
            x => panic!("Bad register {}", x),
        };

        self.registers.a = n;

        match opcode {
            0x3e => self.registers.pc += 2,
            0xfa => self.registers.pc += 3,
            _ => self.registers.pc += 1,
        };
        match opcode {
            0xfa => self.cycles += 16,
            0x0a | 0x1a | 0x3e | 0x7e => self.cycles += 8,
            _ => self.cycles += 4,
        }
    }

    fn ld_c_a(&mut self, memory: &mut Memory) {
        let addr = 0xff00 + self.registers.c as u16;
        memory.set_u8(addr, self.registers.a);
        self.registers.pc += 1;
        self.cycles += 8;
    }

    fn ld_nn_n(&mut self, opcode: u8, memory: &Memory) {
        let dest_index = (opcode & 0b0011_1000) >> 3;
        let value = self.load_imm_u8(memory);
        self.set_dest_u8(dest_index, value);
        self.registers.pc += 2;
        self.cycles += 8;
    }

    fn jr_cc_n(&mut self, opcode: u8, memory: &Memory) {
        let condition = match (opcode & 0b11000) >> 3 {
            0 => !self.registers.flagz(),
            1 => self.registers.flagz(),
            2 => !self.registers.flagc(),
            3 => self.registers.flagc(),
            x => panic!("Bad condition {}", x),
        };

        if condition {
            let v = self.load_imm_u8(memory);
            self.registers.pc = signed_add_u16_u8(self.registers.pc + 2, v);
            self.cycles += 12;
        } else {
            self.registers.pc += 2;
            self.cycles += 8;
        }
    }

    fn ldd_hl_a(&mut self, memory: &mut Memory) {
        let hl = self.registers.hld();
        memory.set_u8(hl, self.registers.a);
        self.registers.pc += 1;
        self.cycles += 8;
    }

    fn xor(&mut self, opcode: u8, memory: &Memory) {
        let source_index = opcode & 0b111;
        let x = self.get_source_u8(source_index, memory);

        self.registers.a ^= x;
        self.registers.clear_flags();
        let flagz = self.registers.a == 0;
        self.registers.set_flagz(flagz);
        self.registers.pc += 1;
        match opcode {
            0xae | 0xee => self.cycles += 8,
            _ => self.cycles += 4,
        }
    }

    fn ld_n_nn(&mut self, opcode: u8, memory: &Memory) {
        let reg_index = (opcode & 0b0011_0000) >> 4;
        let value = self.load_imm_u16(memory);
        match reg_index {
            0 => self.registers.set_bc(value),
            1 => self.registers.set_de(value),
            2 => self.registers.set_hl(value),
            3 => self.registers.sp = value,
            _ => panic!("Bad register {}", reg_index),
        }
        self.registers.pc += 3;
        self.cycles += 12;
    }

    fn bit_b_r(&mut self, opcode: u8, memory: &Memory) {
        let source_index = opcode & 0b111;
        let x = self.get_source_u8(source_index, memory);
        let shift = (opcode & 0b111000) >> 3;

        let t = x & (1 << shift);

        self.registers.f &= 0b0001_0000;
        self.registers.set_flagh(true);
        self.registers.set_flagz(t == 0);

        self.registers.pc += 1;
        match opcode {
            0x46 => self.cycles += 16,
            _ => self.cycles += 8,
        }
    }
}

// Add a 'signed' u8 to an unsigned u16
fn signed_add_u16_u8(lhs: u16, rhs: u8) -> u16 {
    if rhs & 0b1000_0000 != 0 {
        lhs - (!rhs + 1) as u16
    } else {
        lhs + rhs as u16
    }
}
