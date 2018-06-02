use super::lcd_registers::LCDRegisters;
use cartridge::Cartridge;

const KILOBYTE: usize = 1024;

const HRAM_START: usize = 0xff80;
const HRAM_SIZE: usize = 127;
const HRAM_END: usize = HRAM_START + HRAM_SIZE - 1;

const VRAM_START: usize = 0x8000;
const VRAM_SIZE: usize = 8 * KILOBYTE;
const VRAM_END: usize = VRAM_START + VRAM_SIZE - 1;

pub struct Memory<'a> {
    boot_rom: &'a mut [u8],
    cartridge: &'a mut Cartridge,
    hram: [u8; HRAM_SIZE],
    lcd_registers: LCDRegisters,
    vram: [u8; VRAM_SIZE],
}

impl<'a> Memory<'a> {
    pub fn new(boot_rom: &'a mut [u8], cartridge: &'a mut Cartridge) -> Memory<'a> {
        Memory {
            boot_rom,
            cartridge,
            hram: [0; HRAM_SIZE],
            lcd_registers: LCDRegisters::new(),
            vram: [0; VRAM_SIZE],
        }
    }

    fn set_io(&mut self, index: usize, value: u8) {
        match index {
            0xff40...0xff6b => self.lcd_registers.set(index, value),
            _ => (), //println!("{} = {:#08b}", index_to_location(index), value),
        }

        // if index == 0xff40 {
        //     self.print_tiles();
        // }
    }

    fn print_tiles(&self) {
        let lcdc = self.lcd_registers.get(0xff40);
        let start = if lcdc & 0b0001_0000 != 0 {
            0x8000 - VRAM_START
        } else {
            0x8800 - VRAM_START
        };

        for i in 0..=255 {
            let tile_address = start + i * 16;
            self.print_tile(tile_address);
        }
    }

    fn print_tile(&self, addr: usize) {
        let tile = &self.vram[addr..addr+16];
        if tile.iter().any(|&x| x > 0) {
            println!("tile at {} has data", addr / 16);
            println!("------------------");
            for bytes in tile.chunks(2) {
                print!("|");
                for i in 0..8 {
                    let i = 7 - i;
                    let a = (bytes[0] >> i) & 1;
                    let b = ((bytes[1] >> i) & 1) << 1;
                    print!("{}", if a | b > 0 { "XX" } else { "  " });
                }
                println!("|");
            }
            println!("------------------");
        }
    }

    fn set_vram(&mut self, index: usize, value: u8) {
        let vram_index = index - VRAM_START;
        let tile_index = vram_index / 16;

        self.vram[vram_index] = value;

        // let location = index_to_location(index);
        // println!("{} = {:#010b}", location, value);
    }

    pub fn set_u8(&mut self, index: u16, value: u8) {
        let index = index as usize;
        match index {
            0x0...0x7FFF => panic!("Cannot write to {}", index_to_location(index)),
            VRAM_START...VRAM_END => self.set_vram(index, value),
            0xff00...0xff7f => self.set_io(index, value),
            HRAM_START...HRAM_END => self.hram[index - HRAM_START] = value,
            x => {
                let location = index_to_location(x);
                panic!("Bad read: {}", location);
            }
        }
        //
        // 0x0000-0x3FFF 16KB ROM Bank 00
        // 0x4000-0x7FFF 16KB ROM Bank 01..NN
        // 0x8000-0x9FFF 8KB Video RAM (VRAM)
        // 0xA000-0xBFFF 8KB External RAM
        // 0xC000-0xCFFF 4KB Work RAM Bank 0 (WRAM)
        // 0xD000-0xDFFF 4KB Work RAM Bank 1 (WRAM)
        // 0xE000-0xFDFF Same as C000-DDFF (ECHO)
        // 0xFE00-0xFE9F Sprite Attribute Table (OAM)
        // 0xFEA0-0xFEFF Not Usable
        // 0xFF00-0xFF7F I/O Ports
        // 0xFF80-0xFFFE High RAM (HRAM)
        // 0xFFFF        Interrupt Enable Register
    }

    pub fn get_u8(&self, index: u16) -> u8 {
        let index = index as usize;
        match index {
            x if x < self.boot_rom.len() => self.boot_rom[x],
            0x0...0x7fff => self.cartridge.get_u8(index),
            0xff00...0xff7f => 0,
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
            HRAM_START...HRAM_END => {
                set_u16(&mut self.hram, index - HRAM_START, value);
            }
            x => {
                let location = index_to_location(x);
                panic!("Bad write: {}", location);
            }
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
        0xFFFF => format!("InterruptEnableRegister"),
        _ => panic!("Bad index 0x{:x}", index),
    }
}

fn get_u16(mem: &[u8], index: usize) -> u16 {
    let high = mem[index + 1] as u16;
    let low = mem[index] as u16;
    (high << 8) | low
}

fn set_u16(mem: &mut [u8], index: usize, value: u16) {
    let high = value >> 8;
    let low = value & 0xff;
    mem[index + 1] = high as u8;
    mem[index] = low as u8;
}
