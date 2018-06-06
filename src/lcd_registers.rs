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
}
