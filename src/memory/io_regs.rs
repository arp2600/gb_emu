pub const JOYP: usize = 0xff00;
pub const SB: usize = 0xff01;
pub const DIV: usize = 0xff04;
pub const TIMA: usize = 0xff05;
pub const TMA: usize = 0xff06;
pub const TAC: usize = 0xff07;
pub const IF: usize = 0xff0f;
pub const IE: usize = 0xffff;
pub const LCDC: usize = 0xff40;
pub const STAT: usize = 0xff41;
pub const SCY: usize = 0xff42;
pub const SCX: usize = 0xff43;
pub const LY: usize = 0xff44;
pub const LYC: usize = 0xff45;
pub const WY: usize = 0xff4a;
pub const WX: usize = 0xff4b;
pub const DMA: usize = 0xff46;
pub const BGP: usize = 0xff47;
pub const OBP0: usize = 0xff48;
pub const OBP1: usize = 0xff49;
// HDMA1 = 0xff51,
// HDMA2 = 0xff52,
// HDMA3 = 0xff53,
// HDMA4 = 0xff54,
// HDMA5 = 0xff55,
// BCPS = 0xff68,
// BCPD = 0xff69,
// OCPS = 0xff6a,
// OCPD = 0xff6b,
pub const BOOT_ROM_DISABLE: usize = 0xff50;
pub const SC: usize = 0xff02;
pub const NR52: usize = 0xff26;
pub const NR51: usize = 0xff25;
pub const NR50: usize = 0xff24;
pub const NR12: usize = 0xff12;
pub const NR22: usize = 0xff17;
pub const NR42: usize = 0xff21;
pub const NR14: usize = 0xff14;
pub const NR24: usize = 0xff19;
pub const NR44: usize = 0xff23;
pub const NR10: usize = 0xff10;
pub const NR30: usize = 0xff1a;
pub const NR11: usize = 0xff11;
pub const NR13: usize = 0xff13;