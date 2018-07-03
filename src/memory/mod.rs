pub mod io_regs;
pub mod locations;
pub mod joypad;
mod video_memory;
mod sizes;
use bit_ops::BitGetSet;
use cartridge::Cartridge;
use self::locations::*;
pub use self::video_memory::VideoMemory;
pub use self::joypad::JoyPad;

pub struct Memory {
    boot_rom: Vec<u8>,
    boot_rom_enabled: bool,
    cartridge: Cartridge,
    vram: VideoMemory,
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
            vram: VideoMemory::new(),
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io: [0; IO_SIZE],
            interrupt_enable_register: 0,
            serial_data: Vec::new(),
            joypad: JoyPad::new(),
        }
    }

    pub fn get_video_memory(&mut self) -> &mut VideoMemory {
        &mut self.vram
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
        let start_address = source as u16 * 0x100;
        for i in 0..OAM_SIZE {
            let v = self.get_u8(start_address + i as u16);
            self.set_u8((OAM_START + i) as u16, v);
        }
    }

    fn set_io_indexed(&mut self, index: usize, value: u8) {
        match index {
            io_regs::JOYP => self.joypad.set_u8(value),
            io_regs::DMA => self.dma_transfer(value),
            0xff01 => self.serial_data.push(value),
            0xff50 => self.boot_rom_enabled = false,
            _ => (),
        }

        if index == io_regs::STAT {
            self.io[index - IO_START] = value & 0b0111_1000;
        } else {
            self.io[index - IO_START] = value;
        }
    }

    fn get_io_indexed(&self, index: usize) -> u8 {
        let value = self.io[index - IO_START];
        match index {
            io_regs::JOYP => self.joypad.get_u8(),
            _ => value,
        }
    }

    pub fn get_io(&self, reg: usize) -> u8 {
        match reg {
            // IoRegs is not in normal io range
            // so is not accessible in get_io_indexed
            io_regs::IE => self.interrupt_enable_register,
            _ => self.get_io_indexed(reg as usize),
        }
    }

    pub fn set_io(&mut self, reg: usize, value: u8) {
        match reg {
            // IoRegs is not in normal io range
            // so is not accessible in set_io_indexed
            io_regs::IE => self.interrupt_enable_register = value,
            _ => self.set_io_indexed(reg as usize, value),
        }
    }

    pub fn set_u8(&mut self, index: u16, value: u8) {
        let index = index as usize;
        match index {
            ROM_0_START...ROM_0_END => (),
            ROM_N_START...ROM_N_END => (),
            VRAM_START...VRAM_END => self.vram[index] = value,
            EXRAM_START...EXRAM_END => unimplemented!(),
            WRAM_START...WRAM_END => self.wram[index - WRAM_START] = value,
            WRAM_ECHO_START...WRAM_ECHO_END => {
                self.wram[index - WRAM_ECHO_START] = value;
            }
            OAM_START...OAM_END => {
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
            VRAM_START...VRAM_END => self.vram[index],
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
            VRAM_START...VRAM_END => self.vram.get_u16(index),
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
                self.vram.set_u16(index, value);
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
