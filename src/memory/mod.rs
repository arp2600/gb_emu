pub mod io_regs;
pub mod joypad;
pub mod locations;
mod sizes;
mod video_memory;
pub use self::joypad::JoyPad;
pub use self::video_memory::VideoMemory;
use bit_ops::BitGetSet;
use cartridge::Cartridge;
use std::collections::HashSet;

// this will only print once
// useful for not spamming duplicate warnings
macro_rules! eprintln_once {
    ($($args:tt)*) => {
        unsafe {
            static mut PRINTED: bool = false;
            if !PRINTED {
                eprintln!($($args)*);
                PRINTED = true;
            }
        }
    };
}
// this will only print once per key
// useful for not spamming duplicate warnings
macro_rules! eprintln_once_per_key {
    ($key:expr, $type:ty, $($args:tt)*) => {
        use std::sync::Mutex;
        lazy_static! {
            static ref PRINTED_MAP: Mutex<HashSet<$type>> = {
                Mutex::new(HashSet::new())
            };
        }
        let mut map = PRINTED_MAP.lock().unwrap();
        if !map.contains(&$key) {
            eprintln!($($args)*);
            map.insert($key);
        }
    };
}

pub struct Memory {
    boot_rom: Vec<u8>,
    boot_rom_enabled: bool,
    cartridge: Cartridge,
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
    pub fn new(boot_rom: Vec<u8>, cartridge: Cartridge) -> Memory {
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
        for i in 0..sizes::OAM {
            let v = self.get_u8(start_address + i as u16);
            self.set_u8((locations::OAM_START + i) as u16, v);
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
            io_regs::SCY => self.vram.regs.scy,
            io_regs::BGP => self.vram.regs.bgp,
            io_regs::OBP0 => self.vram.regs.obp0,
            io_regs::OBP1 => self.vram.regs.obp1,
            io_regs::IF => {
                let mut x = self.interrupt_flag;
                if self.vram.regs.vblank_interrupt_enabled {
                    x = x.set_bit(0);
                }
                x
            }
            io_regs::DIV | io_regs::TIMA | io_regs::TMA | io_regs::TAC => {
                self.io[index - locations::IO_START]
            }
            _ => {
                eprintln_once_per_key!(
                    index,
                    usize,
                    "warning: reading from placeholder io {}",
                    io_reg_name(index)
                );
                self.io[index - locations::IO_START]
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
                self.vram.regs.stat = new_stat;
            }
            io_regs::LCDC => self.vram.regs.lcdc = value,
            io_regs::LY => self.vram.regs.ly = value,
            io_regs::LYC => self.vram.regs.lyc = value,
            io_regs::SCY => self.vram.regs.scy = value,
            io_regs::BGP => self.vram.regs.bgp = value,
            io_regs::OBP0 => self.vram.regs.obp0 = value,
            io_regs::OBP1 => self.vram.regs.obp1 = value,
            io_regs::IF => {
                self.interrupt_flag = value & 0b1111_1110;
                self.vram.regs.vblank_interrupt_enabled = (value & 1) != 0;
            }
            io_regs::DIV | io_regs::TIMA | io_regs::TMA | io_regs::TAC => {
                self.io[index - locations::IO_START] = value;
            }
            _ => {
                eprintln_once_per_key!(
                    index,
                    usize,
                    "warning: writing {:#04x} to placeholder io {}",
                    value,
                    io_reg_name(index)
                );
                self.io[index - locations::IO_START] = value;
            }
        }
    }

    pub fn set_u8(&mut self, index: u16, value: u8) {
        let index = index as usize;
        match index {
            locations::ROM_0_START...locations::ROM_0_END => (),
            locations::ROM_N_START...locations::ROM_N_END => (),
            locations::VRAM_START...locations::VRAM_END => self.vram[index] = value,
            locations::EXRAM_START...locations::EXRAM_END => unimplemented!(),
            locations::WRAM_START...locations::WRAM_END => {
                self.wram[index - locations::WRAM_START] = value
            }
            locations::WRAM_ECHO_START...locations::WRAM_ECHO_END => {
                self.wram[index - locations::WRAM_ECHO_START] = value;
            }
            locations::OAM_START...locations::OAM_END => {
                self.vram[index] = value;
            }
            locations::IO_START...locations::IO_END => self.set_io(index, value),
            locations::HRAM_START...locations::HRAM_END => {
                self.hram[index - locations::HRAM_START] = value
            }
            locations::INTERRUPT_ENABLE_REG => {
                self.interrupt_enable_register = value;
                if value.get_bit(1) {
                    eprintln_once!("warning: Lcd STAT interrupt not implemented");
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
            locations::ROM_0_START...locations::ROM_0_END => self.cartridge.get_u8(index),
            locations::ROM_N_START...locations::ROM_N_END => self.cartridge.get_u8(index),
            locations::VRAM_START...locations::VRAM_END => self.vram[index],
            locations::EXRAM_START...locations::EXRAM_END => unimplemented!(),
            locations::WRAM_START...locations::WRAM_END => self.wram[index - locations::WRAM_START],
            locations::WRAM_ECHO_START...locations::WRAM_ECHO_END => {
                self.wram[index - locations::WRAM_ECHO_START]
            }
            locations::OAM_START...locations::OAM_END => self.vram[index],
            locations::IO_START...locations::IO_END => self.get_io(index),
            locations::HRAM_START...locations::HRAM_END => self.hram[index - locations::HRAM_START],
            locations::INTERRUPT_ENABLE_REG => self.interrupt_enable_register,
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
            locations::ROM_0_START...locations::ROM_0_END => self.cartridge.get_u16(index),
            locations::ROM_N_START...locations::ROM_N_END => self.cartridge.get_u16(index),
            locations::VRAM_START...locations::VRAM_END => self.vram.get_u16(index),
            locations::EXRAM_START...locations::EXRAM_END => unimplemented!(),
            locations::WRAM_START...locations::WRAM_END => {
                get_u16(&self.wram, index - locations::WRAM_START)
            }
            locations::WRAM_ECHO_START...locations::WRAM_ECHO_END => {
                get_u16(&self.wram, index - locations::WRAM_ECHO_START)
            }
            locations::OAM_START...locations::OAM_END => self.vram.get_u16(index),
            locations::HRAM_START...locations::HRAM_END => {
                get_u16(&self.hram, index - locations::HRAM_START)
            }
            locations::INTERRUPT_ENABLE_REG => unimplemented!(),
            x => {
                let location = index_to_location(x);
                panic!("Bad read: {}", location);
            }
        }
    }

    pub fn set_u16(&mut self, index: u16, value: u16) {
        let index = index as usize;
        match index {
            locations::ROM_0_START...locations::ROM_0_END => bad_write_panic(index),
            locations::ROM_N_START...locations::ROM_N_END => bad_write_panic(index),
            locations::VRAM_START...locations::VRAM_END => {
                self.vram.set_u16(index, value);
            }
            locations::EXRAM_START...locations::EXRAM_END => unimplemented!(),
            locations::WRAM_START...locations::WRAM_END => {
                set_u16(&mut self.wram, index - locations::WRAM_START, value);
            }
            locations::WRAM_ECHO_START...locations::WRAM_ECHO_END => {
                set_u16(&mut self.wram, index - locations::WRAM_ECHO_START, value);
            }
            locations::OAM_START...locations::OAM_END => {
                if value != 0 {
                    println!("OAM {:#06x} set to {:#06x}", index, value);
                }
                self.vram.set_u16(index, value);
            }
            locations::HRAM_START...locations::HRAM_END => {
                set_u16(&mut self.hram, index - locations::HRAM_START, value);
            }
            locations::INTERRUPT_ENABLE_REG => unimplemented!(),
            x => bad_write_panic(x),
        }
    }

    fn is_valid_boot_rom_index(&self, index: usize) -> bool {
        self.boot_rom_enabled && index < self.boot_rom.len()
    }
}

pub fn index_to_location(index: usize) -> String {
    match index {
        locations::ROM_0_START...locations::ROM_0_END => format!("ROM bank 0[0x{:x}]", index),
        locations::ROM_N_START...locations::ROM_N_END => format!("ROM bank n[0x{:x}]", index),
        locations::VRAM_START...locations::VRAM_END => format!("VRAM[0x{:x}]", index),
        locations::EXRAM_START...locations::EXRAM_END => format!("EXRAM[0x{:x}]", index),
        locations::WRAM_START...locations::WRAM_END => format!("WRAM[0x{:x}]", index),
        locations::WRAM_ECHO_START...locations::WRAM_ECHO_END => format!("ECHO[0x{:x}]", index),
        locations::OAM_START...locations::OAM_END => format!("OAM[0x{:x}]", index),
        0xfea0...0xfeff => format!("Not usable[0x{:x}]", index),
        locations::IO_START...locations::IO_END => format!("IO[0x{:x}]", index),
        locations::HRAM_START...locations::HRAM_END => format!("HRAM[0x{:x}]", index),
        locations::INTERRUPT_ENABLE_REG => String::from("InterruptEnableRegister"),
        _ => panic!("Bad index 0x{:x}", index),
    }
}

fn io_reg_name(index: usize) -> String {
    match index {
        io_regs::SCX => format!("SCX - Scroll X"),
        io_regs::SC => format!("SC - Serial Transfer Control"),
        io_regs::NR52 => format!("NR52 - Sound on/off"),
        io_regs::NR51 => format!("NR51 - Selection of Sound output terminal"),
        io_regs::NR50 => format!("NR50 - Channel control / ON-OFF / Volume"),
        io_regs::NR12 => format!("NR12 - Channel 1 Volume Envelope"),
        io_regs::NR22 => format!("NR22 - Channel 2 Volume Envelope"),
        io_regs::NR42 => format!("NR42 - Channel 4 Volume Envelope"),
        io_regs::NR14 => format!("NR14 - Channel 1 Frequency hi"),
        io_regs::NR24 => format!("NR24 - Channel 2 Frequency hi data"),
        io_regs::NR44 => format!("NR44 - Channel 4 Counter/consecutive; Inital"),
        io_regs::NR10 => format!("NR10 - Channel 1 Sweep register"),
        io_regs::NR30 => format!("NR30 - Channel 3 Sound on/off"),
        io_regs::WY => format!("WY - Window Y Position"),
        io_regs::WX => format!("WX - Window X Position minus 7"),
        io_regs::NR11 => format!("NR11 - Channel 1 Sound length/Wave pattern duty"),
        io_regs::NR13 => format!("NR13 - Channel 1 Frequency lo"),
        _ => format!("{:#06x}", index),
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
