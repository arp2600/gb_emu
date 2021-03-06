pub mod io_regs;
pub mod joypad;
pub mod locations;
pub mod sizes;
mod video_memory;
pub use self::joypad::JoyPad;
use self::locations::*;
pub use self::video_memory::VideoMemory;
use crate::bit_ops::BitGetSet;
use crate::cartridge::Cartridge;
use std::collections::HashSet;

pub struct Memory {
    boot_rom: Vec<u8>,
    boot_rom_enabled: bool,
    cartridge: Box<Cartridge>,
    vram: VideoMemory,
    wram: [u8; sizes::WRAM],
    io: [u8; sizes::IO],
    hram: [u8; sizes::HRAM],
    interrupt_enable_register: u8,
    serial_data: Vec<u8>,
    joypad: JoyPad,
    interrupt_flag: u8,
}

impl Memory {
    pub fn new(boot_rom: Vec<u8>, cartridge: Box<Cartridge>) -> Memory {
        Memory {
            boot_rom,
            boot_rom_enabled: true,
            cartridge,
            hram: [0; sizes::HRAM],
            vram: VideoMemory::new(),
            wram: [0; sizes::WRAM],
            io: [0; sizes::IO],
            interrupt_enable_register: 0,
            serial_data: Vec::new(),
            joypad: JoyPad::new(),
            interrupt_flag: 0,
        }
    }

    pub fn get_video_memory(&mut self) -> &mut VideoMemory {
        &mut self.vram
    }

    pub fn get_cartridge(&self) -> &Cartridge {
        &*self.cartridge
    }

    pub fn get_cartridge_mut(&mut self) -> &mut Cartridge {
        &mut *self.cartridge
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

    fn dma_transfer(&mut self, source: u8) {
        let start_address = u16::from(source) * 0x100;
        for i in 0..sizes::OAM {
            let v = self.get_u8(start_address + i as u16);
            self.set_u8((OAM_START + i) as u16, v);
        }
    }

    pub fn get_io(&self, index: usize) -> u8 {
        match index {
            io_regs::IE => self.interrupt_enable_register,
            io_regs::JOYP => self.joypad.get_u8(),
            io_regs::LCDC => self.vram.regs.lcdc,
            io_regs::LY => self.vram.regs.ly,
            io_regs::LYC => self.vram.regs.lyc,
            io_regs::STAT => self.vram.regs.stat,
            io_regs::WY => self.vram.regs.wy,
            io_regs::WX => self.vram.regs.wx,
            io_regs::SCY => self.vram.regs.scy,
            io_regs::SCX => self.vram.regs.scx,
            io_regs::BGP => self.vram.regs.bgp,
            io_regs::OBP0 => self.vram.regs.obp0,
            io_regs::OBP1 => self.vram.regs.obp1,
            io_regs::IF => {
                let mut x = self.interrupt_flag;
                if self.vram.regs.vblank_interrupt_enabled {
                    x = x.set_bit(0);
                }
                if self.vram.regs.stat_interrupt_enabled {
                    x = x.set_bit(1);
                }
                x
            }
            io_regs::DIV | io_regs::TIMA | io_regs::TMA | io_regs::TAC => self.io[index - IO_START],
            _ => {
                eprintln_once_per_key!(
                    index,
                    usize,
                    "warning: reading from placeholder io {}",
                    io_regs::io_reg_description(index)
                );
                self.io[index - IO_START]
            }
        }
    }

    pub fn set_io(&mut self, index: usize, value: u8) {
        match index {
            io_regs::JOYP => self.joypad.set_u8(value),
            io_regs::DMA => self.dma_transfer(value),
            io_regs::IE => self.interrupt_enable_register = value,
            io_regs::SB => self.serial_data.push(value),
            io_regs::BOOT_ROM_DISABLE => self.boot_rom_enabled = false,
            io_regs::STAT => {
                let stat = self.vram.regs.stat;
                let new_stat = (stat & 0b0111) | (value & 0b0111_1000);
                if new_stat.get_bit(5) {
                    eprintln_once!("warning: OAM interrupt not implemented");
                }
                if new_stat.get_bit(4) {
                    eprintln_once!("warning: v-blank interrupt not implemented");
                }
                if new_stat.get_bit(3) {
                    eprintln_once!("warning: h-blank interrupt not implemented");
                }
                self.vram.regs.stat = new_stat;
            }
            io_regs::LCDC => self.vram.regs.lcdc = value,
            io_regs::LY => self.vram.regs.ly = value,
            io_regs::LYC => self.vram.regs.lyc = value,
            io_regs::WY => self.vram.regs.wy = value,
            io_regs::WX => self.vram.regs.wx = value,
            io_regs::SCY => self.vram.regs.scy = value,
            io_regs::SCX => self.vram.regs.scx = value,
            io_regs::BGP => self.vram.regs.bgp = value,
            io_regs::OBP0 => self.vram.regs.obp0 = value,
            io_regs::OBP1 => self.vram.regs.obp1 = value,
            io_regs::IF => {
                self.interrupt_flag = value & 0b1111_1110;
                self.vram.regs.vblank_interrupt_enabled = value.get_bit(0);
                self.vram.regs.stat_interrupt_enabled = value.get_bit(1);
            }
            io_regs::DIV | io_regs::TIMA | io_regs::TMA | io_regs::TAC => {
                self.io[index - IO_START] = value;
            }
            _ => {
                eprintln_once_per_key!(
                    index,
                    usize,
                    "warning: writing {:#04x} to placeholder io {}",
                    value,
                    io_regs::io_reg_description(index)
                );
                self.io[index - IO_START] = value;
            }
        }
    }

    pub fn set_u8(&mut self, index: u16, value: u8) {
        let index = index as usize;
        match index {
            ROM_0_START...ROM_0_END => self.cartridge.set_u8(index, value),
            ROM_N_START...ROM_N_END => self.cartridge.set_u8(index, value),
            VRAM_START...VRAM_END => self.vram[index] = value,
            EXRAM_START...EXRAM_END => self.cartridge.set_u8(index, value),
            WRAM_START...WRAM_END => self.wram[index - WRAM_START] = value,
            WRAM_ECHO_START...WRAM_ECHO_END => {
                self.wram[index - WRAM_ECHO_START] = value;
            }
            OAM_START...OAM_END => {
                self.vram[index] = value;
            }
            IO_START...IO_END => self.set_io(index, value),
            HRAM_START...HRAM_END => self.hram[index - HRAM_START] = value,
            INTERRUPT_ENABLE_REG => {
                self.interrupt_enable_register = value;
                if value.get_bit(1) {
                    eprintln_once!("warning: Lcd STAT interrupt only partially implemented");
                } else if value.get_bit(3) {
                    eprintln_once!("warning: Serial interrupt not implemented");
                } else if value.get_bit(4) {
                    eprintln_once!("warning: Joypad interrupt not implemented");
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
            VRAM_START...VRAM_END => self.vram[index],
            EXRAM_START...EXRAM_END => self.cartridge.get_u8(index),
            WRAM_START...WRAM_END => self.wram[index - WRAM_START],
            WRAM_ECHO_START...WRAM_ECHO_END => self.wram[index - WRAM_ECHO_START],
            OAM_START...OAM_END => self.vram[index],
            IO_START...IO_END => self.get_io(index),
            HRAM_START...HRAM_END => self.hram[index - HRAM_START],
            INTERRUPT_ENABLE_REG => self.interrupt_enable_register,
            x => {
                let location = index_to_location(x);
                panic!("Bad read: {}", location);
            }
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
