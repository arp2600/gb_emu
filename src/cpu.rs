use super::memory::Memory;
use super::registers::Registers;

pub struct Cpu {
    registers: Registers,
    instruction_counter: usize,
    interrupts_enabled: bool,
    cycles: u64,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Default::default(),
            instruction_counter: 0,
            interrupts_enabled: false,
            cycles: 0,
        }
    }

    pub fn skip_boot_rom(&mut self) {
        self.registers.set_af(0x01b0);
        self.registers.set_bc(0x0013);
        self.registers.set_de(0x00d8);
        self.registers.set_hl(0x014d);
        self.registers.sp = 0xfffe;
        self.registers.pc = 0x0100;
    }

    pub fn get_cycles(&self) -> u64 {
        self.cycles
    }

    pub fn get_registers(&self) -> &Registers {
        &self.registers
    }

    pub fn tick(&mut self, memory: &mut Memory, tracing: bool) {
        self.instruction_counter += 1;
        let mnemonic = if tracing {
            Some(self.get_opcode_mnemonic(memory))
        } else {
            None
        };
        let opcode = memory.get_u8(self.registers.pc);

        self.fetch_and_execute(memory);

        if tracing {
            println!("{:020}|  {:#04x}", mnemonic.unwrap(), opcode);
        }
    }

    fn get_opcode_mnemonic(&self, memory: &Memory) -> String {
        let opcode = memory.get_u8(self.registers.pc);

        match opcode {
            0x00 => "nop".to_string(),
            0x01 => format!("ld BC, {:#06x}", self.load_imm_u16(memory)),
            0x11 => format!("ld DE, {:#06x}", self.load_imm_u16(memory)),
            0x21 => format!("ld HL, {:#06x}", self.load_imm_u16(memory)),
            0x31 => format!("ld SP, {:#06x}", self.load_imm_u16(memory)),
            0x20 => format!("jr NZ, {}", self.load_imm_u8(memory) as i8),
            0x28 => format!("jr Z, {}", self.load_imm_u8(memory) as i8),
            0x30 => format!("jr NC, {}", self.load_imm_u8(memory) as i8),
            0x38 => format!("jr C, {}", self.load_imm_u8(memory) as i8),
            0x06 => format!("ld B, {:#04x}", self.load_imm_u8(memory)),
            0x0e => format!("ld C, {:#04x}", self.load_imm_u8(memory)),
            0x16 => format!("ld D, {:#04x}", self.load_imm_u8(memory)),
            0x1e => format!("ld E, {:#04x}", self.load_imm_u8(memory)),
            0x26 => format!("ld H, {:#04x}", self.load_imm_u8(memory)),
            0x2e => format!("ld L, {:#04x}", self.load_imm_u8(memory)),
            0x3c => format!("inc A({:#04x})", self.registers.a),
            0x04 => format!("inc B({:#04x})", self.registers.b),
            0x0c => format!("inc C({:#04x})", self.registers.c),
            0x14 => format!("inc D({:#04x})", self.registers.d),
            0x1c => format!("inc E({:#04x})", self.registers.e),
            0x24 => format!("inc H({:#04x})", self.registers.h),
            0x2c => format!("inc L({:#04x})", self.registers.l),
            0x34 => format!("inc (HL({:#04x}))", self.registers.get_hl()),
            0x47 => format!("ld B, {:#04x}", self.registers.a),
            0x4f => format!("ld C, {:#04x}", self.registers.a),
            0x57 => format!("ld D, {:#04x}", self.registers.a),
            0x5f => format!("ld E, {:#04x}", self.registers.a),
            0x67 => format!("ld H, {:#04x}", self.registers.a),
            0x6f => format!("ld L, {:#04x}", self.registers.a),
            0x02 => format!("ld (BC), {:#04x}", self.registers.a),
            0x12 => format!("ld (DE), {:#04x}", self.registers.a),
            0x77 => format!("ld (HL), {:#04x}", self.registers.a),
            0xea => format!(
                "ld ({:#06x}), {:#04x}",
                self.load_imm_u16(memory),
                self.registers.a
            ),
            0xe0 => format!(
                "ldh ({:#04x}), {:#04}",
                0xff00 + u16::from(self.load_imm_u8(memory)),
                self.registers.a
            ),
            0x18 => format!("jr {:#04x}", self.load_imm_u8(memory) as i8),
            0xc3 => format!("jp {:#06x}", self.load_imm_u16(memory)),
            0x2a => format!("ldi A, (HL({:#06x}))", self.registers.get_hl()),
            0x32 => format!(
                "ldd (HL{:#06x}), A{:#04x}",
                self.registers.get_hl(),
                self.registers.a
            ),
            0x0a => format!("ld A, (BC({:#06x}))", self.registers.get_bc()),
            0x1a => format!("ld A, (DE({:#06x}))", self.registers.get_de()),
            0x3e => format!("ld A, {:#04x}", self.load_imm_u8(memory)),
            0x78 => format!("ld A, B({:#04x})", self.registers.b),
            0x79 => format!("ld A, C({:#04x})", self.registers.c),
            0x7a => format!("ld A, D({:#04x})", self.registers.d),
            0x7b => format!("ld A, E({:#04x})", self.registers.e),
            0x7c => format!("ld A, H({:#04x})", self.registers.h),
            0x7d => format!("ld A, L({:#04x})", self.registers.l),
            0x7e => format!("ld A, (HL({:#06x}))", self.registers.get_hl()),
            0x7f => format!("ld A, A({:#04x})", self.registers.a),
            0xfa => format!("ld A, ({:#06x})", self.load_imm_u16(memory)),
            0xe2 => format!("ld (C), A({:#04x})", self.registers.a),
            0xcd => format!("call {:#06x}", self.load_imm_u16(memory)),
            0xf5 => "push AF".to_string(),
            0xc5 => "push BC".to_string(),
            0xd5 => "push DE".to_string(),
            0xe5 => "push HL".to_string(),
            0xf1 => "pop AF".to_string(),
            0xc1 => "pop BC".to_string(),
            0xd1 => "pop DE".to_string(),
            0xe1 => "pop HL".to_string(),
            0x17 => "rla".to_string(),
            0x3d => "dec A".to_string(),
            0x05 => "dec B".to_string(),
            0x0d => "dec C".to_string(),
            0x15 => "dec D".to_string(),
            0x1d => "dec E".to_string(),
            0x25 => "dec H".to_string(),
            0x2d => "dec L".to_string(),
            0x35 => "dec (HL)".to_string(),
            0x22 => format!(
                "ldi (HL({:#06x}), A({:#04x})",
                self.registers.get_hl(),
                self.registers.a
            ),
            0x03 => "inc BC".to_string(),
            0x13 => "inc DE".to_string(),
            0x23 => "inc HL".to_string(),
            0x33 => "inc SP".to_string(),
            0xc9 => "ret".to_string(),
            0xb8 => format!("cp B({:#04x})", self.registers.b),
            0xb9 => format!("cp C({:#04x})", self.registers.c),
            0xba => format!("cp D({:#04x})", self.registers.d),
            0xbb => format!("cp E({:#04x})", self.registers.e),
            0xbc => format!("cp H({:#04x})", self.registers.h),
            0xbd => format!("cp L({:#04x})", self.registers.l),
            0xbe => format!("cp (HL({:#06x}))", self.registers.get_hl()),
            0xbf => format!("cp A({:#04x})", self.registers.a),
            0xfe => format!("cp {:#04x}", self.load_imm_u8(memory)),
            0xf0 => format!(
                "ldh A, ({:#06x})",
                0xff00 + u16::from(self.load_imm_u8(memory))
            ),
            0xf3 => "di".to_string(),
            0xfb => "ei".to_string(),
            0x1f => "rra".to_string(),
            0xe9 => format!("jp (HL({:#06x}))", self.registers.get_hl()),
            0x90 => format!("sub A, B({:#04x})", self.registers.b),
            0x91 => format!("sub A, C({:#04x})", self.registers.c),
            0x92 => format!("sub A, D({:#04x})", self.registers.d),
            0x93 => format!("sub A, E({:#04x})", self.registers.e),
            0x94 => format!("sub A, H({:#04x})", self.registers.h),
            0x95 => format!("sub A, L({:#04x})", self.registers.l),
            0x96 => format!("sub A, (HL({:#06x}))", self.registers.get_hl()),
            0x97 => format!("sub A, A({:#04x})", self.registers.a),
            0xd6 => format!("sub A, {:#04x}", self.load_imm_u8(memory)),
            0x80 => format!("add A, B({:#04x})", self.registers.b),
            0x81 => format!("add A, C({:#04x})", self.registers.c),
            0x82 => format!("add A, D({:#04x})", self.registers.d),
            0x83 => format!("add A, E({:#04x})", self.registers.e),
            0x84 => format!("add A, H({:#04x})", self.registers.h),
            0x85 => format!("add A, L({:#04x})", self.registers.l),
            0x86 => format!("add A, (HL({:#06x}))", self.registers.get_hl()),
            0x87 => format!("add A, A({:#04x})", self.registers.a),
            0xc6 => format!("add A, {:#04x}", self.load_imm_u8(memory)),
            0xb7 => format!("or A, A({:#04x})", self.registers.a),
            0xb0 => format!("or A, B({:#04x})", self.registers.b),
            0xb1 => format!("or A, C({:#04x})", self.registers.c),
            0xb2 => format!("or A, D({:#04x})", self.registers.d),
            0xb3 => format!("or A, E({:#04x})", self.registers.e),
            0xb4 => format!("or A, H({:#04x})", self.registers.h),
            0xb5 => format!("or A, L({:#04x})", self.registers.l),
            0xb6 => format!("or A, (HL({:#06x}))", self.registers.get_hl()),
            0xf6 => format!("or A, {:#04x}", self.load_imm_u8(memory)),
            0xa7 => format!("and A, A({:#04x})", self.registers.a),
            0xa0 => format!("and A, B({:#04x})", self.registers.b),
            0xa1 => format!("and A, C({:#04x})", self.registers.c),
            0xa2 => format!("and A, D({:#04x})", self.registers.d),
            0xa3 => format!("and A, E({:#04x})", self.registers.e),
            0xa4 => format!("and A, H({:#04x})", self.registers.h),
            0xa5 => format!("and A, L({:#04x})", self.registers.l),
            0xa6 => format!("and A, (HL({:#06x}))", self.registers.get_hl()),
            0xe6 => format!("and A, {:#04x}", self.load_imm_u8(memory)),
            0xaf => format!("xor A, A({:#04x})", self.registers.a),
            0xa8 => format!("xor A, B({:#04x})", self.registers.b),
            0xa9 => format!("xor A, C({:#04x})", self.registers.c),
            0xaa => format!("xor A, D({:#04x})", self.registers.d),
            0xab => format!("xor A, E({:#04x})", self.registers.e),
            0xac => format!("xor A, H({:#04x})", self.registers.h),
            0xad => format!("xor A, L({:#04x})", self.registers.l),
            0xae => format!("xor A, (HL({:#06x}))", self.registers.get_hl()),
            0xee => format!("xor A, {:#04x}", self.load_imm_u8(memory)),
            0xc4 => format!("call NZ, {:#06x}", self.load_imm_u16(memory)),
            0xcc => format!("call Z, {:#06x}", self.load_imm_u16(memory)),
            0xd4 => format!("call NC, {:#06x}", self.load_imm_u16(memory)),
            0xdc => format!("call C, {:#06x}", self.load_imm_u16(memory)),
            0x8f => format!("adc A, A({:#04x})", self.registers.a),
            0x88 => format!("adc A, B({:#04x})", self.registers.b),
            0x89 => format!("adc A, C({:#04x})", self.registers.c),
            0x8a => format!("adc A, D({:#04x})", self.registers.d),
            0x8b => format!("adc A, E({:#04x})", self.registers.e),
            0x8c => format!("adc A, H({:#04x})", self.registers.h),
            0x8d => format!("adc A, L({:#04x})", self.registers.l),
            0x8e => format!("adc A, (HL({:#06x}))", self.registers.get_hl()),
            0xce => format!("adc A, {:#04x}", self.load_imm_u8(memory)),
            0xc0 => "ret NZ".to_string(),
            0xc8 => "ret Z".to_string(),
            0xd0 => "ret NC".to_string(),
            0xd8 => "ret C".to_string(),
            0x09 => format!("add HL, BC({:#06x})", self.registers.get_bc()),
            0x19 => format!("add HL, DE({:#06x})", self.registers.get_de()),
            0x29 => format!("add HL, HL({:#06x})", self.registers.get_hl()),
            0x39 => format!("ADD HL, SP({:#06x})", self.registers.sp),
            0xc2 => format!("jp NZ, {:#06x}", self.load_imm_u16(memory)),
            0xca => format!("jp Z, {:#06x}", self.load_imm_u16(memory)),
            0xd2 => format!("jp NC, {:#06x}", self.load_imm_u16(memory)),
            0xda => format!("JP C, {:#06x}", self.load_imm_u16(memory)),
            0x68 => format!("ld L, B({:#04x})", self.registers.b),
            0x69 => format!("ld L, C({:#04x})", self.registers.c),
            0x6a => format!("ld L, D({:#04x})", self.registers.d),
            0x6b => format!("ld L, E({:#04x})", self.registers.e),
            0x6c => format!("ld L, H({:#04x})", self.registers.h),
            0x6d => format!("ld L, L({:#04x})", self.registers.l),
            0x6e => format!("ld L, (HL({:#06x}))", self.registers.get_hl()),
            0xcb => self.get_cb_opcode_mnemonic(memory),
            _ => "__".to_string(),
        }
    }

    fn get_cb_opcode_mnemonic(&self, memory: &Memory) -> String {
        let opcode = memory.get_u8(self.registers.pc + 1);

        match opcode {
            0x40...0x7f => "BIT".to_string(),
            0x10...0x17 => "RL".to_string(),
            _ => "CB__".to_string(),
        }
    }

    fn fetch_and_execute(&mut self, memory: &mut Memory) {
        let opcode = memory.get_u8(self.registers.pc);

        match opcode {
            0x00 => self.nop(),
            0xcb => self.fetch_and_execute_cb(memory),
            0x01 | 0x11 | 0x21 | 0x31 => self.ld_n_nn(opcode, memory),
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
            0x80...0x87 | 0xc6 => self.add_a_n(opcode, memory),
            0xc3 => self.jp_nn(memory),
            0xf3 => self.set_interrupts(false),
            0xfb => self.set_interrupts(true),
            0x2a => self.ldi_a_hl(memory),
            0xb0...0xb7 | 0xf6 => self.or_n(opcode, memory),
            0xa0...0xa7 | 0xe6 => self.and_n(opcode, memory),
            0xa8...0xaf | 0xee => self.xor_n(opcode, memory),
            0xc4 | 0xcc | 0xd4 | 0xdc => self.call_cc_nn(opcode, memory),
            0x1f => self.rra(),
            0x88...0x8f | 0xce => self.adc_a_n(opcode, memory),
            0xc0 | 0xc8 | 0xd0 | 0xd8 => self.ret_cc(opcode, memory),
            0x68...0x6e => self.ld_l_n(opcode, memory),
            0x9 | 0x19 | 0x29 | 0x39 => self.add_hl_n(opcode),
            0xe9 => self.jp_hl(),
            0xc2 | 0xca | 0xd2 | 0xda => self.jp_cc_nn(opcode, memory),
            _ => panic!("Instruction 0x{:02x} not implemented", opcode),
        }
    }

    fn fetch_and_execute_cb(&mut self, memory: &mut Memory) {
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

    fn set_dest_u8(&mut self, index: u8, value: u8, memory: &mut Memory) {
        match index {
            0 => self.registers.b = value,
            1 => self.registers.c = value,
            2 => self.registers.d = value,
            3 => self.registers.e = value,
            4 => self.registers.h = value,
            5 => self.registers.l = value,
            6 => {
                let hl = self.registers.get_hl();
                memory.set_u8(hl, value);
            }
            7 => self.registers.a = value,
            _ => panic!("Bad register {}", index),
        }
    }

    fn load_imm_u8(&self, memory: &Memory) -> u8 {
        memory.get_u8(self.registers.pc + 1)
    }

    fn load_imm_u16(&self, memory: &Memory) -> u16 {
        memory.get_u16(self.registers.pc + 1)
    }

    /************************************************************
                         Opcodes
    ************************************************************/

    fn jp_cc_nn(&mut self, opcode: u8, memory: &Memory) {
        let nn = self.load_imm_u16(memory);
        let cc = match opcode {
            0xc2 => !self.registers.flagz(),
            0xca => self.registers.flagz(),
            0xd2 => !self.registers.flagc(),
            0xda => self.registers.flagc(),
            _ => panic!("Bad opcode {}", opcode),
        };

        if cc {
            self.registers.pc = nn;
            self.cycles += 16;
        } else {
            self.registers.pc += 3;
            self.cycles += 12;
        }
    }

    fn jp_hl(&mut self) {
        self.registers.pc = self.registers.get_hl();
        self.cycles += 4;
    }

    fn add_hl_n(&mut self, opcode: u8) {
        let n = match opcode {
            0x09 => self.registers.get_bc(),
            0x19 => self.registers.get_de(),
            0x29 => self.registers.get_hl(),
            0x39 => self.registers.sp,
            _ => panic!("Bad opcode {}", opcode),
        };

        let hl = self.registers.get_hl();
        let result = hl.wrapping_add(n);
        self.registers.set_hl(result);

        self.registers.set_flagn(false);
        self.registers.set_flagh((hl & 0xfff) + (n & 0xfff) > 0xfff);
        self.registers
            .set_flagc(u32::from(hl) + u32::from(n) > 0xffff);

        self.registers.pc += 1;
        self.cycles += 8;
    }

    fn ld_l_n(&mut self, opcode: u8, memory: &mut Memory) {
        self.registers.l = match opcode {
            0x68...0x6e => {
                let reg_index = opcode & 0b0000_0111;
                self.get_source_u8(reg_index, memory)
            }
            _ => panic!("Bad opcode {}", opcode),
        };

        self.registers.pc += 1;
        match opcode {
            0x6e => self.cycles += 8,
            _ => self.cycles += 4,
        }
    }

    fn ret_cc(&mut self, opcode: u8, memory: &mut Memory) {
        let cc = match opcode {
            0xc0 => !self.registers.flagz(),
            0xc8 => self.registers.flagz(),
            0xd0 => !self.registers.flagc(),
            0xd8 => self.registers.flagc(),
            _ => panic!("Bad opcode {}", opcode),
        };

        if cc {
            self.registers.pc = memory.get_u16(self.registers.sp);
            self.registers.sp += 2;
            self.cycles += 20;
        } else {
            self.registers.pc += 1;
            self.cycles += 8;
        }
    }

    fn adc_a_n(&mut self, opcode: u8, memory: &mut Memory) {
        let mut n = match opcode {
            0x88...0x8f => {
                let reg_index = opcode & 0b0000_0111;
                self.get_source_u8(reg_index, memory)
            }
            0xce => {
                let n = self.load_imm_u8(memory);
                self.registers.pc += 1;
                n
            }
            _ => panic!("Bad opcode {}", opcode),
        };

        if self.registers.flagc() {
            n += 1;
        }

        let a = self.registers.a;
        let result = a.wrapping_add(n);
        self.registers.a = result;

        self.registers.clear_flags();
        self.registers.set_flagz(result == 0);
        self.registers.set_flagh((a & 0xf) + (n & 0xf) > 0xf);
        self.registers.set_flagc(u16::from(a) + u16::from(n) > 255);

        self.registers.pc += 1;
        match opcode {
            0x8e | 0xce => self.cycles += 8,
            _ => self.cycles += 4,
        }
    }

    fn rra(&mut self) {
        let a = self.registers.a;
        let mut result = a >> 1;
        if self.registers.flagc() {
            result += 0b1000_0000;
        }

        self.registers.clear_flags();
        self.registers.set_flagz(result == 0);
        self.registers.set_flagz(a & 0b1 == 1);

        self.registers.a = result;

        self.registers.pc += 1;
        self.cycles += 4;
    }

    fn call_cc_nn(&mut self, opcode: u8, memory: &mut Memory) {
        let cc = match opcode {
            0xc4 => !self.registers.flagz(),
            0xcc => self.registers.flagz(),
            0xd4 => !self.registers.flagc(),
            0xdc => self.registers.flagc(),
            _ => panic!("Bad opcode {}", opcode),
        };

        self.registers.pc += 3;

        if cc {
            let nn = self.load_imm_u16(memory);
            memory.set_u16(self.registers.sp - 2, self.registers.pc);
            self.registers.sp -= 2;
            self.registers.pc = nn;
            self.cycles += 24;
        } else {
            self.cycles += 12;
        }
    }

    fn xor_n(&mut self, opcode: u8, memory: &Memory) {
        let n = match opcode {
            0xa8...0xaf => {
                let reg_index = opcode & 0b0000_0111;
                self.get_source_u8(reg_index, memory)
            }
            0xee => {
                let n = self.load_imm_u8(memory);
                self.registers.pc += 1;
                n
            }
            _ => panic!("Bad opcode {}", opcode),
        };

        let result = n ^ self.registers.a;
        self.registers.a = result;
        self.registers.clear_flags();
        self.registers.set_flagz(result == 0);

        self.registers.pc += 1;
        match opcode {
            0xae | 0xee => self.cycles += 8,
            _ => self.cycles += 4,
        }
    }

    fn and_n(&mut self, opcode: u8, memory: &Memory) {
        let n = match opcode {
            0xa0...0xa7 => {
                let reg_index = opcode & 0b0000_0111;
                self.get_source_u8(reg_index, memory)
            }
            0xe6 => {
                let n = self.load_imm_u8(memory);
                self.registers.pc += 1;
                n
            }
            _ => panic!("Bad opcode {}", opcode),
        };

        let result = n & self.registers.a;
        self.registers.a = result;
        self.registers.clear_flags();
        self.registers.set_flagz(result == 0);
        self.registers.set_flagh(true);

        self.registers.pc += 1;
        match opcode {
            0xa6 | 0xe6 => self.cycles += 8,
            _ => self.cycles += 4,
        }
    }

    fn or_n(&mut self, opcode: u8, memory: &Memory) {
        let n = match opcode {
            0xb0...0xb7 => {
                let reg_index = opcode & 0b0000_0111;
                self.get_source_u8(reg_index, memory)
            }
            0xf6 => {
                let n = self.load_imm_u8(memory);
                self.registers.pc += 1;
                n
            }
            _ => panic!("Bad opcode {}", opcode),
        };

        let result = n | self.registers.a;
        self.registers.a = result;
        self.registers.clear_flags();
        self.registers.set_flagz(result == 0);

        self.registers.pc += 1;
        match opcode {
            0xb6 | 0xf6 => self.cycles += 8,
            _ => self.cycles += 4,
        }
    }

    fn ldi_a_hl(&mut self, memory: &Memory) {
        let hl = self.registers.hli();
        let v = memory.get_u8(hl);
        self.registers.a = v;

        self.registers.pc += 1;
        self.cycles += 8;
    }

    fn set_interrupts(&mut self, state: bool) {
        self.interrupts_enabled = state;
        self.registers.pc += 1;
        self.cycles += 4;
    }

    fn jp_nn(&mut self, memory: &Memory) {
        let nn = self.load_imm_u16(memory);
        self.registers.pc = nn;
        self.cycles += 12;
    }

    fn nop(&mut self) {
        self.registers.pc += 1;
        self.cycles += 4;
    }

    fn add_a_n(&mut self, opcode: u8, memory: &Memory) {
        let n = match opcode {
            0x80...0x87 => {
                let reg_index = opcode & 0b0000_0111;
                self.get_source_u8(reg_index, memory)
            }
            0xc6 => {
                let n = self.load_imm_u8(memory);
                self.registers.pc += 1;
                n
            }
            _ => panic!("Bad opcode {}", opcode),
        };

        let a = self.registers.a;
        let result = a.wrapping_add(n);
        self.registers.a = result;

        self.registers.set_flagz(result == 0);
        self.registers.set_flagn(false);
        self.registers.set_flagh((a & 0xf) + (n & 0xf) > 0xf);
        self.registers.set_flagc(u16::from(a) + u16::from(n) > 255);

        self.registers.pc += 1;
        match opcode {
            0x86 | 0xc6 => self.cycles += 8,
            _ => self.cycles += 4,
        }
    }

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
        self.registers.a = a.wrapping_sub(n);

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
        let v = memory.get_u8(0xff00 + u16::from(n));
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

    fn dec_n(&mut self, opcode: u8, memory: &mut Memory) {
        let reg_index = (opcode & 0b0011_1000) >> 3;
        let source = self.get_source_u8(reg_index, memory);
        let result = source.wrapping_sub(1);

        self.registers.set_flagz(result == 0);
        self.registers.set_flagn(true);
        self.registers.set_flagh(source.trailing_zeros() >= 4);

        self.set_dest_u8(reg_index, result, memory);
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

    fn rl_n(&mut self, opcode: u8, memory: &mut Memory) {
        let reg_index = opcode & 0b0000_0111;
        let source = self.get_source_u8(reg_index, memory);
        let mut value = source << 1;
        if self.registers.flagc() {
            value += 1;
        };

        self.registers.clear_flags();
        self.registers.set_flagz(value == 0);
        self.registers.set_flagc((source & 0b1000_0000) != 0);

        self.set_dest_u8(reg_index, value, memory);
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
        let addr = u16::from(self.load_imm_u8(memory)) + 0xff00;
        memory.set_u8(addr, self.registers.a);
        self.registers.pc += 2;
        self.cycles += 12;
    }

    fn ld_n_a(&mut self, opcode: u8, memory: &mut Memory) {
        let value = self.registers.a;
        match opcode {
            0x7f | 0x47 | 0x4f | 0x57 | 0x5f | 0x67 | 0x6f => {
                let reg_index = (opcode & 0b0011_1000) >> 3;
                self.set_dest_u8(reg_index, value, memory);
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

    fn inc_n(&mut self, opcode: u8, memory: &mut Memory) {
        let reg_index = (opcode & 0b11_1000) >> 3;
        let source = self.get_source_u8(reg_index, memory);
        let result = source.wrapping_add(1);
        self.registers.set_flagz(result == 0);
        self.registers.set_flagn(false);
        self.registers.set_flagh((source & 0xf) + 1 > 0xf);
        self.set_dest_u8(reg_index, result, memory);
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
        let addr = 0xff00 + u16::from(self.registers.c);
        memory.set_u8(addr, self.registers.a);
        self.registers.pc += 1;
        self.cycles += 8;
    }

    fn ld_nn_n(&mut self, opcode: u8, memory: &mut Memory) {
        let dest_index = (opcode & 0b0011_1000) >> 3;
        let value = self.load_imm_u8(memory);
        self.set_dest_u8(dest_index, value, memory);
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
        let shift = (opcode & 0b11_1000) >> 3;

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
        lhs - u16::from(!rhs + 1)
    } else {
        lhs + u16::from(rhs)
    }
}
