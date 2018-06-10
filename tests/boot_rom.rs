// extern crate gb_emu;
// use gb_emu::Emulator;
//
// #[test]
// fn post_boot_rom_state() {
//     let boot_rom = "../ROMs/dmg_rom.gb";
//     let cartridge_rom = "../ROMs/tetris.gb";
//     let mut emulator = Emulator::new(Some(&boot_rom), &cartridge_rom);
//     // Prevent infinite loops
//     for _ in 0..3000_000 {
//         emulator.tick();
//         if !emulator.is_boot_rom_enabled() {
//             break;
//         }
//     }
//
//     let registers = emulator.get_registers();
//
//     assert_eq!(registers.get_af(), 0x01b0);
//     assert_eq!(registers.get_bc(), 0x0013);
//     assert_eq!(registers.get_de(), 0x00d8);
//     assert_eq!(registers.get_hl(), 0x014d);
//     assert_eq!(registers.sp, 0xfffe);
//     assert_eq!(registers.pc, 0x0100);
// }
