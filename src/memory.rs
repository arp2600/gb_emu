use super::bit_ops::BitGetSet;
use cartridge::Cartridge;
use memory_values::*;

pub struct JoyPad {
    buttons: u8,
    directions: u8,
    // 0x20 for for buttons, 0x10 for directions
    // 0x30 and 0x00 ???
    selection: u8,
}

fn set_bit(x: u8, bit: u8, state: bool) -> u8 {
    if state {
        x | (0b1 << bit)
    } else {
        x & !(0b1 << bit)
    }
}

impl JoyPad {
    pub fn set_a(&mut self, state: bool) {
        self.buttons = set_bit(self.buttons, 0, !state);
    }

    pub fn set_b(&mut self, state: bool) {
        self.buttons = set_bit(self.buttons, 1, !state);
    }

    pub fn set_select(&mut self, state: bool) {
        self.buttons = set_bit(self.buttons, 2, !state);
    }

    pub fn set_start(&mut self, state: bool) {
        self.buttons = set_bit(self.buttons, 3, !state);
    }

    pub fn set_right(&mut self, state: bool) {
        self.directions = set_bit(self.directions, 0, !state);
    }

    pub fn set_left(&mut self, state: bool) {
        self.directions = set_bit(self.directions, 1, !state);
    }

    pub fn set_up(&mut self, state: bool) {
        self.directions = set_bit(self.directions, 2, !state);
    }

    pub fn set_down(&mut self, state: bool) {
        self.directions = set_bit(self.directions, 3, !state);
    }

    fn new() -> JoyPad {
        JoyPad {
            buttons: 0x0f,
            directions: 0x0f,
            selection: 0x30,
        }
    }

    fn set_u8(&mut self, value: u8) {
        self.selection = value & 0b0011_0000;
    }

    fn get_u8(&self) -> u8 {
        let p14 = self.selection.get_bit(4);
        let p15 = self.selection.get_bit(5);

        if !p15 && p14 {
            0b1100_1111 & self.buttons
        } else if p15 && !p14 {
            0b1100_1111 & self.directions
        } else {
            0b1100_1111
        }
    }
}

pub struct Memory {
    boot_rom: Vec<u8>,
    boot_rom_enabled: bool,
    cartridge: Cartridge,
    vram: [u8; VRAM_SIZE],
    wram: [u8; WRAM_SIZE],
    oam: [u8; OAM_SIZE],
    io: [u8; IO_SIZE],
    hram: [u8; HRAM_SIZE],
    interrupt_enable_register: u8,
    serial_data: Vec<u8>,
    joypad: JoyPad,
}

impl Memory {
    pub fn new(boot_rom: Vec<u8>, cartridge: Cartridge) -> Memory {
        Memory {
            boot_rom,
            boot_rom_enabled: true,
            cartridge,
            hram: [0; HRAM_SIZE],
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io: [0; IO_SIZE],
            interrupt_enable_register: 0,
            serial_data: Vec::new(),
            joypad: JoyPad::new(),
        }
    }

    pub fn set_stat(&mut self, value: u8) {
        self.io[STAT - IO_START] = value;
    }

    pub fn get_joypad(&mut self) -> &mut JoyPad {
        &mut self.joypad
    }

    pub fn get_serial_data(&self) -> &[u8] {
        &self.serial_data
    }

    pub fn is_boot_rom_enabled(&self) -> bool {
        self.boot_rom_enabled
    }

    fn set_io_indexed(&mut self, index: usize, value: u8) {
        match index {
            JOYP => self.joypad.set_u8(value),
            0xff01 => self.serial_data.push(value),
            0xff50 => self.boot_rom_enabled = false,
            _ => (),
        }

        if index == STAT {
            self.io[index - IO_START] = value & 0b0111_1000;
        } else {
            self.io[index - IO_START] = value;
        }
    }

    fn get_io_indexed(&self, index: usize) -> u8 {
        let value = self.io[index - IO_START];
        match index {
            JOYP => self.joypad.get_u8(),
            _ => value,
        }
    }

    pub fn get_io(&self, reg: IoRegs) -> u8 {
        match reg {
            // IoRegs is not in normal io range
            // so is not accessible in get_io_indexed
            IoRegs::IE => self.interrupt_enable_register,
            _ => self.get_io_indexed(reg as usize),
        }
    }

    pub fn set_io(&mut self, reg: IoRegs, value: u8) {
        match reg {
            // IoRegs is not in normal io range
            // so is not accessible in set_io_indexed
            IoRegs::IE => self.interrupt_enable_register = value,
            _ => self.set_io_indexed(reg as usize, value),
        }
    }

    pub fn set_u8(&mut self, index: u16, value: u8) {
        let index = index as usize;
        match index {
            ROM_0_START...ROM_0_END => (),
            ROM_N_START...ROM_N_END => (),
            VRAM_START...VRAM_END => self.vram[index - VRAM_START] = value,
            EXRAM_START...EXRAM_END => unimplemented!(),
            WRAM_START...WRAM_END => self.wram[index - WRAM_START] = value,
            WRAM_ECHO_START...WRAM_ECHO_END => {
                self.wram[index - WRAM_ECHO_START] = value;
            }
            OAM_START...OAM_END => {
                if value != 0 {
                    println!("OAM {:#06x} set to {:#04x}", index, value);
                }
                self.oam[index - OAM_START] = value;
            }
            IO_START...IO_END => self.set_io_indexed(index, value),
            HRAM_START...HRAM_END => self.hram[index - HRAM_START] = value,
            INTERRUPT_ENABLE_REG => {
                self.interrupt_enable_register = value;
                if value.get_bit(1) {
                    eprintln!("Warning: Lcd STAT interrupt not implemented");
                } else if value.get_bit(3) {
                    eprintln!("Warning: Serial interrupt not implemented");
                } else if value.get_bit(4) {
                    eprintln!("Warning: Joypad interrupt not implemented");
                }
            }
            _ => (),
        }
    }

    pub fn get_u8(&self, index: u16) -> u8 {
        let index = index as usize;
        match index {
            x if self.is_valid_boot_rom_index(x) => self.boot_rom[x],
            ROM_0_START...ROM_0_END => self.cartridge.get_u8(index),
            ROM_N_START...ROM_N_END => self.cartridge.get_u8(index),
            VRAM_START...VRAM_END => self.vram[index - VRAM_START],
            EXRAM_START...EXRAM_END => unimplemented!(),
            WRAM_START...WRAM_END => self.wram[index - WRAM_START],
            WRAM_ECHO_START...WRAM_ECHO_END => self.wram[index - WRAM_ECHO_START],
            OAM_START...OAM_END => self.oam[index - OAM_START],
            IO_START...IO_END => self.get_io_indexed(index),
            HRAM_START...HRAM_END => self.hram[index - HRAM_START],
            INTERRUPT_ENABLE_REG => self.interrupt_enable_register,
            x => {
                let location = index_to_location(x);
                panic!("Bad read: {}", location);
            }
        }
    }

    pub fn get_u16(&self, index: u16) -> u16 {
        let index = index as usize;
        match index {
            x if self.is_valid_boot_rom_index(x) => get_u16(&self.boot_rom, index),
            ROM_0_START...ROM_0_END => self.cartridge.get_u16(index),
            ROM_N_START...ROM_N_END => self.cartridge.get_u16(index),
            VRAM_START...VRAM_END => get_u16(&self.vram, index - VRAM_START),
            EXRAM_START...EXRAM_END => unimplemented!(),
            WRAM_START...WRAM_END => get_u16(&self.wram, index - WRAM_START),
            WRAM_ECHO_START...WRAM_ECHO_END => get_u16(&self.wram, index - WRAM_ECHO_START),
            OAM_START...OAM_END => get_u16(&self.oam, index - OAM_START),
            HRAM_START...HRAM_END => get_u16(&self.hram, index - HRAM_START),
            INTERRUPT_ENABLE_REG => unimplemented!(),
            x => {
                let location = index_to_location(x);
                panic!("Bad read: {}", location);
            }
        }
    }

    pub fn set_u16(&mut self, index: u16, value: u16) {
        let index = index as usize;
        match index {
            ROM_0_START...ROM_0_END => bad_write_panic(index),
            ROM_N_START...ROM_N_END => bad_write_panic(index),
            VRAM_START...VRAM_END => {
                set_u16(&mut self.vram, index - VRAM_START, value);
            }
            EXRAM_START...EXRAM_END => unimplemented!(),
            WRAM_START...WRAM_END => {
                set_u16(&mut self.wram, index - WRAM_START, value);
            }
            WRAM_ECHO_START...WRAM_ECHO_END => {
                set_u16(&mut self.wram, index - WRAM_ECHO_START, value);
            }
            OAM_START...OAM_END => {
                if value != 0 {
                    println!("OAM {:#06x} set to {:#06x}", index, value);
                }
                set_u16(&mut self.oam, index - OAM_START, value);
            }
            HRAM_START...HRAM_END => {
                set_u16(&mut self.hram, index - HRAM_START, value);
            }
            INTERRUPT_ENABLE_REG => unimplemented!(),
            x => bad_write_panic(x),
        }
    }

    fn is_valid_boot_rom_index(&self, index: usize) -> bool {
        self.boot_rom_enabled && index < self.boot_rom.len()
    }
}

pub fn index_to_location(index: usize) -> String {
    match index {
        ROM_0_START...ROM_0_END => format!("ROM bank 0[0x{:x}]", index),
        ROM_N_START...ROM_N_END => format!("ROM bank n[0x{:x}]", index),
        VRAM_START...VRAM_END => format!("VRAM[0x{:x}]", index),
        EXRAM_START...EXRAM_END => format!("EXRAM[0x{:x}]", index),
        WRAM_START...WRAM_END => format!("WRAM[0x{:x}]", index),
        WRAM_ECHO_START...WRAM_ECHO_END => format!("ECHO[0x{:x}]", index),
        OAM_START...OAM_END => format!("OAM[0x{:x}]", index),
        0xfea0...0xfeff => format!("Not usable[0x{:x}]", index),
        IO_START...IO_END => format!("IO[0x{:x}]", index),
        HRAM_START...HRAM_END => format!("HRAM[0x{:x}]", index),
        INTERRUPT_ENABLE_REG => String::from("InterruptEnableRegister"),
        _ => panic!("Bad index 0x{:x}", index),
    }
}

fn get_u16(mem: &[u8], index: usize) -> u16 {
    let high = u16::from(mem[index + 1]);
    let low = u16::from(mem[index]);
    (high << 8) | low
}

fn set_u16(mem: &mut [u8], index: usize, value: u16) {
    let high = value >> 8;
    let low = value & 0xff;
    mem[index + 1] = high as u8;
    mem[index] = low as u8;
}

fn bad_write_panic(index: usize) {
    panic!("Cannot write to {}", index_to_location(index));
}
