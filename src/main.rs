mod memory;
mod opcode_table;
mod opcodes;
mod registers;
use memory::Memory;
use opcode_table::OPCODE_TABLE;
use registers::Registers;
use std::fs;

fn get_boot_rom() -> Vec<u8> {
    fs::read("../ROMs/dmg_rom.gb").unwrap()
}

fn main() {
    let mut boot_rom = get_boot_rom();
    let mut registers = Registers::new();
    let mut mem = Memory::new(&mut boot_rom);

    loop {
        let opcode = match mem.get_u8(registers.pc) {
            0xcb => (mem.get_u8(registers.pc + 1) as usize) + 0x100,
            x => x as usize,
        };

        let opcode_function = OPCODE_TABLE[opcode as usize];
        opcode_function(&mut registers, &mut mem);
        println!("{:?}", registers);
    }
}
