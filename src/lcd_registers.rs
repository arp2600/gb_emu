#[derive(Debug)]
pub struct LCDRegisters {
    pub lcdc: u8,
    pub stat: u8,
    pub sy: u8,
    pub sx: u8,
    pub ly: u8,
    pub lyc: u8,
    pub wy: u8,
    pub wx: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub bcps: u8,
    pub bcpd: u8,
    pub ocps: u8,
    pub ocpd: u8,
    pub vbk: u8,
    pub dma: u8,
    pub hdma1: u8,
    pub hdma2: u8,
    pub hdma3: u8,
    pub hdma4: u8,
    pub hdma5: u8,
}

impl LCDRegisters {
    pub fn new() -> LCDRegisters {
        LCDRegisters {
            lcdc: 0,
            stat: 0,
            sy: 0,
            sx: 0,
            ly: 0,
            lyc: 0,
            wy: 0,
            wx: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            bcps: 0,
            bcpd: 0,
            ocps: 0,
            ocpd: 0,
            vbk: 0,
            dma: 0,
            hdma1: 0,
            hdma2: 0,
            hdma3: 0,
            hdma4: 0,
            hdma5: 0,
        }
    }

    pub fn set(&mut self, index: usize, value: u8) {
        match index {
            0xff40 => self.lcdc = value,
            0xff41 => self.stat = value,
            0xff42 => self.sy = value,
            0xff43 => self.sx = value,
            0xff44 => self.ly = value,
            0xff45 => self.lyc = value,
            0xff46 => self.dma = value,
            0xff47 => self.bgp = value,
            0xff48 => self.obp0 = value,
            0xff49 => self.obp1 = value,
            0xff4a => self.wy = value,
            0xff4b => self.wx = value,
            0xff4f => self.vbk = value,
            0xff51 => self.hdma1 = value,
            0xff52 => self.hdma2 = value,
            0xff53 => self.hdma3 = value,
            0xff54 => self.hdma4 = value,
            0xff55 => self.hdma5 = value,
            0xff68 => self.bcps = value,
            0xff69 => self.bcpd = value,
            0xff6a => self.ocps = value,
            0xff6b => self.ocpd = value,
            _ => panic!("0x{:x} is not an lcd register"),
        }

        self.trace_set(index, value);
    }

    fn trace_set(&mut self, index: usize, value: u8) {
        let reg_name = index_to_reg_name(index);
        println!("LCD[{}] = {:#08b}", reg_name, value);
    }
}

fn index_to_reg_name(index: usize) -> &'static str {
    match index {
        0xff40 => "LCDC",
        0xff41 => "STAT",
        0xff42 => "SY",
        0xff43 => "SX",
        0xff44 => "LY",
        0xff45 => "LYC",
        0xff46 => "DMA",
        0xff47 => "BGP",
        0xff48 => "OBP0",
        0xff49 => "OBP1",
        0xff4a => "WY",
        0xff4b => "WX",
        0xff4f => "VBK",
        0xff51 => "HDMA1",
        0xff52 => "HDMA2",
        0xff53 => "HDMA3",
        0xff54 => "HDMA4",
        0xff55 => "HDMA5",
        0xff68 => "BCPS",
        0xff69 => "BCPD",
        0xff6a => "OCPS",
        0xff6b => "OCPD",
        _ => panic!("0x{:x} is not an lcd register"),
    }
}
