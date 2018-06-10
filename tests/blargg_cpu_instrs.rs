extern crate gb_emu;
use gb_emu::Emulator;

#[test]
fn special() {
    run_test_rom("blargg_test_roms/cpu_instrs/individual/01-special.gb");
}

#[test]
fn interrupts() {
    run_test_rom("blargg_test_roms/cpu_instrs/individual/02-interrupts.gb");
}

#[test]
fn op_sp_hl() {
    run_test_rom("blargg_test_roms/cpu_instrs/individual/03-op_sp_hl.gb");
}

#[test]
fn op_r_imm() {
    run_test_rom("blargg_test_roms/cpu_instrs/individual/04-op_r_imm.gb");
}

#[test]
fn op_rp() {
    run_test_rom("blargg_test_roms/cpu_instrs/individual/05-op_rp.gb");
}

#[test]
fn ld_r_r() {
    run_test_rom("blargg_test_roms/cpu_instrs/individual/06-ld_r_r.gb");
}

#[test]
fn jr_jp_call_ret_rst() {
    run_test_rom("blargg_test_roms/cpu_instrs/individual/07-jr_jp_call_ret_rst.gb");
}

#[test]
fn misc_instrs() {
    run_test_rom("blargg_test_roms/cpu_instrs/individual/08-misc_instrs.gb");
}

#[test]
fn op_r_r() {
    run_test_rom("blargg_test_roms/cpu_instrs/individual/09-op_r_r.gb");
}

#[test]
fn bit_ops() {
    run_test_rom("blargg_test_roms/cpu_instrs/individual/10-bit_ops.gb");
}

#[test]
fn op_a_hl() {
    run_test_rom("blargg_test_roms/cpu_instrs/individual/11-op_a_hl.gb");
}

#[test]
fn cpu_instrs() {
    run_test_rom("blargg_test_roms/cpu_instrs/cpu_instrs.gb");
}


fn run_test_rom(test_rom_path: &str) {
    let mut emulator = Emulator::new(None, test_rom_path);
    for _ in 0..3000_000 {
        emulator.tick()
    }
}
