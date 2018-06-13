use super::lcd_registers::LCDRegisters;
use cartridge::Cartridge;
use memory_values::*;
use std::ops::FnMut;

pub struct Memory {
    boot_rom: Vec<u8>,
    boot_rom_enabled: bool,
    cartridge: Cartridge,
    vram: [u8; VRAM_SIZE],
    wram: [u8; WRAM_SIZE],
    oam: [u8; OAM_SIZE],
    hram: [u8; HRAM_SIZE],
    lcd_registers: LCDRegisters,
    interrupt_enable_register: u8,
    serial_io_callback: Option<Box<FnMut(u8)>>,
    last_serial_byte: u8,
    serial_data: Vec<u8>,
}

impl Memory {
    pub fn new(boot_rom: Vec<u8>, cartridge: Cartridge) -> Memory {
        Memory {
            boot_rom,
            boot_rom_enabled: true,
            cartridge,
            hram: [0; HRAM_SIZE],
            lcd_registers: LCDRegisters::new(),
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            interrupt_enable_register: 0,
            serial_io_callback: None,
            last_serial_byte: 0,
            serial_data: Vec::new(),
        }
    }

    pub fn get_serial_data(&self) -> &[u8] {
        &self.serial_data
    }

    pub fn set_serial_io_callback(&mut self, callback: Box<FnMut(u8)>) {
        self.serial_io_callback = Some(callback);
    }

    pub fn is_boot_rom_enabled(&self) -> bool {
        self.boot_rom_enabled
    }

    fn set_io(&mut self, index: usize, value: u8) {
        match index {
            0xff01 => {
                self.last_serial_byte = value;
                self.serial_data.push(value);
            }
            0xff02 => match &mut self.serial_io_callback {
                Some(x) => x(self.last_serial_byte),
                None => (),
            },
            0xff40 => self.lcd_registers.lcdc = value,
            0xff41 => self.lcd_registers.stat = value,
            0xff42 => self.lcd_registers.sy = value,
            0xff43 => self.lcd_registers.sx = value,
            0xff44 => self.lcd_registers.ly = value,
            0xff45 => self.lcd_registers.lyc = value,
            0xff46 => self.lcd_registers.dma = value,
            0xff47 => self.lcd_registers.bgp = value,
            0xff48 => self.lcd_registers.obp0 = value,
            0xff49 => self.lcd_registers.obp1 = value,
            0xff50 => self.boot_rom_enabled = false,
            0xff4a => self.lcd_registers.wy = value,
            0xff4b => self.lcd_registers.wx = value,
            0xff4f => self.lcd_registers.vbk = value,
            0xff51 => self.lcd_registers.hdma1 = value,
            0xff52 => self.lcd_registers.hdma2 = value,
            0xff53 => self.lcd_registers.hdma3 = value,
            0xff54 => self.lcd_registers.hdma4 = value,
            0xff55 => self.lcd_registers.hdma5 = value,
            0xff68 => self.lcd_registers.bcps = value,
            0xff69 => self.lcd_registers.bcpd = value,
            0xff6a => self.lcd_registers.ocps = value,
            0xff6b => self.lcd_registers.ocpd = value,
            _ => (),
        }
    }

    fn get_io(&self, index: usize) -> u8 {
        match index {
            0xff01 => {
                println!("get SB");
                self.last_serial_byte
            }
            0xff02 => {
                println!("get SC");
                0
            }
            0xff40 => self.lcd_registers.lcdc,
            0xff41 => self.lcd_registers.stat,
            0xff42 => self.lcd_registers.sy,
            0xff43 => self.lcd_registers.sx,
            0xff44 => self.lcd_registers.ly,
            0xff45 => self.lcd_registers.lyc,
            0xff46 => self.lcd_registers.dma,
            0xff47 => self.lcd_registers.bgp,
            0xff48 => self.lcd_registers.obp0,
            0xff49 => self.lcd_registers.obp1,
            0xff4a => self.lcd_registers.wy,
            0xff4b => self.lcd_registers.wx,
            0xff4f => self.lcd_registers.vbk,
            0xff51 => self.lcd_registers.hdma1,
            0xff52 => self.lcd_registers.hdma2,
            0xff53 => self.lcd_registers.hdma3,
            0xff54 => self.lcd_registers.hdma4,
            0xff55 => self.lcd_registers.hdma5,
            0xff68 => self.lcd_registers.bcps,
            0xff69 => self.lcd_registers.bcpd,
            0xff6a => self.lcd_registers.ocps,
            0xff6b => self.lcd_registers.ocpd,
            _ => 0,
        }
    }

    pub fn set_u8(&mut self, index: u16, value: u8) {
        let index = index as usize;
        match index {
            ROM_0_START...ROM_0_END => bad_write_panic(index),
            ROM_N_START...ROM_N_END => bad_write_panic(index),
            VRAM_START...VRAM_END => self.vram[index - VRAM_START] = value,
            EXRAM_START...EXRAM_END => unimplemented!(),
            WRAM_START...WRAM_END => self.wram[index - WRAM_START] = value,
            WRAM_ECHO_START...WRAM_ECHO_END => {
                self.wram[index - WRAM_ECHO_START] = value;
            }
            OAM_START...OAM_END => self.oam[index - OAM_START] = value,
            IO_START...IO_END => self.set_io(index, value),
            HRAM_START...HRAM_END => self.hram[index - HRAM_START] = value,
            INTERRUPT_ENABLE_REG => self.interrupt_enable_register = value,
            x => bad_write_panic(x),
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
            IO_START...IO_END => self.get_io(index),
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
