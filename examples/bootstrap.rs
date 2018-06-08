extern crate gb_emu;
use gb_emu::Emulator;
use std::fs;

fn main() {
    let boot_rom = "../ROMs/dmg_rom.gb";
    let cartridge_rom = "../ROMs/tetris.gb";
    let mut emulator = Emulator::new(Some(&boot_rom), &cartridge_rom);
    while emulator.is_boot_rom_enabled() {
        emulator.tick();
    }

    let mut emu2 = Emulator::new(None, &cartridge_rom);
    let r1 = emulator.get_registers();
    let r2 = emulator.get_registers();
    assert_eq!(r1.get_af(), r2.get_af());
    assert_eq!(r1.get_bc(), r2.get_bc());
    assert_eq!(r1.get_de(), r2.get_de());
    assert_eq!(r1.get_hl(), r2.get_hl());
    assert_eq!(r1.sp, r2.sp);
    assert_eq!(r1.pc, r2.pc);

    let registers = emulator.get_registers();
    println!("AF: {:#06x}", registers.get_af());
    println!("BC: {:#06x}", registers.get_bc());
    println!("DE: {:#06x}", registers.get_de());
    println!("HL: {:#06x}", registers.get_hl());
    println!("SP: {:#06x}", registers.sp);
    println!("PC: {:#06x}", registers.pc);
}
