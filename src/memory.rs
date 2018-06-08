use super::lcd_registers::LCDRegisters;
use cartridge::Cartridge;

const KILOBYTE: usize = 1024;

const HRAM_START: usize = 0xff80;
const HRAM_SIZE: usize = 127;
const HRAM_END: usize = HRAM_START + HRAM_SIZE - 1;

const VRAM_START: usize = 0x8000;
const VRAM_SIZE: usize = 8 * KILOBYTE;
const VRAM_END: usize = VRAM_START + VRAM_SIZE - 1;

const IO_START: usize = 0xff00;
const IO_END: usize = 0xff7f;

const WRAM_START: usize = 0xc000;
const WRAM_SIZE: usize = 8 * KILOBYTE;
const WRAM_END: usize = WRAM_START + WRAM_SIZE - 1;
const WRAM_ECHO_START: usize = 0xe000;
const WRAM_ECHO_END: usize = 0xfdff;

const OAM_START: usize = 0xfe00;
const OAM_END: usize = 0xfe9f;
const OAM_SIZE: usize = OAM_END - OAM_START + 1;

pub struct Memory {
    boot_rom: Vec<u8>,
    cartridge: Cartridge,
    hram: [u8; HRAM_SIZE],
    lcd_registers: LCDRegisters,
    vram: [u8; VRAM_SIZE],
    wram: [u8; WRAM_SIZE],
    oam: [u8; OAM_SIZE],
}

impl Memory {
    pub fn new(boot_rom: Vec<u8>, cartridge: Cartridge) -> Memory {
        Memory {
            boot_rom,
            cartridge,
            hram: [0; HRAM_SIZE],
            lcd_registers: LCDRegisters::new(),
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
        }
    }

    fn set_io(&mut self, index: usize, value: u8) {
        match index {
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
            0xff50 => panic!("Disable boot rom"),
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

    fn set_vram(&mut self, index: usize, value: u8) {
        let vram_index = index - VRAM_START;
        self.vram[vram_index] = value;
    }

    pub fn set_u8(&mut self, index: u16, value: u8) {
        let index = index as usize;
        match index {
            0x0...0x7FFF => bad_write_panic(index),
            VRAM_START...VRAM_END => self.vram[index - VRAM_START] = value,
            WRAM_START...WRAM_END => self.wram[index - WRAM_START] = value,
            WRAM_ECHO_START...WRAM_ECHO_END => {
                self.wram[index - WRAM_ECHO_START] = value;
            }
            OAM_START...OAM_END => self.oam[index - OAM_START] = value,
            IO_START...IO_END => self.set_io(index, value),
            HRAM_START...HRAM_END => self.hram[index - HRAM_START] = value,
            x => bad_write_panic(x),
        }
    }

    pub fn get_u8(&self, index: u16) -> u8 {
        let index = index as usize;
        match index {
            x if x < self.boot_rom.len() => self.boot_rom[x],
            0x0...0x7fff => self.cartridge.get_u8(index),
            WRAM_START...WRAM_END => self.wram[index - WRAM_START],
            WRAM_ECHO_START...WRAM_ECHO_END => self.wram[index - WRAM_ECHO_START],
            OAM_START...OAM_END => self.oam[index - OAM_START],
            IO_START...IO_END => self.get_io(index),
            HRAM_START...HRAM_END => self.hram[index - HRAM_START],
            x => {
                let location = index_to_location(x);
                panic!("Bad read: {}", location);
            }
        }
    }

    pub fn get_u16(&self, index: u16) -> u16 {
        let index = index as usize;
        match index {
            x if x < self.boot_rom.len() => get_u16(&self.boot_rom, index),
            0x0...0x7fff => self.cartridge.get_u16(index),
            WRAM_START...WRAM_END => get_u16(&self.wram, index - WRAM_START),
            WRAM_ECHO_START...WRAM_ECHO_END => get_u16(&self.wram, index - WRAM_ECHO_START),
            OAM_START...OAM_END => get_u16(&self.oam, index - OAM_START),
            HRAM_START...HRAM_END => get_u16(&self.hram, index - HRAM_START),
            x => {
                let location = index_to_location(x);
                panic!("Bad read: {}", location);
            }
        }
    }

    pub fn set_u16(&mut self, index: u16, value: u16) {
        let index = index as usize;
        match index {
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
            x => bad_write_panic(x),
        }
    }
}

pub fn index_to_location(index: usize) -> String {
    match index {
        0x0000...0x3FFF => format!("ROM bank 0[0x{:x}]", index),
        0x4000...0x7FFF => format!("ROM bank n[0x{:x}]", index),
        VRAM_START...VRAM_END => format!("VRAM[0x{:x}]", index),
        0xA000...0xBFFF => format!("RAM[0x{:x}]", index),
        0xC000...0xCFFF => format!("WRAM0[0x{:x}]", index),
        0xD000...0xDFFF => format!("WRAM1[0x{:x}]", index),
        0xE000...0xFDFF => format!("ECHO[0x{:x}]", index),
        0xFE00...0xFE9F => format!("OAM[0x{:x}]", index),
        0xFEA0...0xFEFF => format!("Not usable[0x{:x}]", index),
        0xFF00...0xFF7F => format!("IO[0x{:x}]", index),
        HRAM_START...HRAM_END => format!("HRAM[0x{:x}]", index),
        0xFFFF => String::from("InterruptEnableRegister"),
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
