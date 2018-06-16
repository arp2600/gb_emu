extern crate gb_emu;
use gb_emu::Emulator;
use std::path::Path;
use std::str;

#[test]
fn special() {
    run_test_rom("individual/01-special.gb", 3_000_000);
}

#[test]
fn interrupts() {
    run_test_rom("individual/02-interrupts.gb", 3_000_000);
}

#[test]
fn op_sp_hl() {
    run_test_rom("individual/03-op_sp_hl.gb", 3_000_000);
}

#[test]
fn op_r_imm() {
    run_test_rom("individual/04-op_r_imm.gb", 3_000_000);
}

#[test]
fn op_rp() {
    run_test_rom("individual/05-op_rp.gb", 3_000_000);
}

#[test]
fn ld_r_r() {
    run_test_rom("individual/06-ld_r_r.gb", 3_000_000);
}

#[test]
fn jr_jp_call_ret_rst() {
    run_test_rom("individual/07-jr_jp_call_ret_rst.gb", 3_000_000);
}

#[test]
fn misc_instrs() {
    run_test_rom("individual/08-misc_instrs.gb", 3_000_000);
}

#[test]
fn op_r_r() {
    run_test_rom("individual/09-op_r_r.gb", 3_000_000);
}

#[test]
fn bit_ops() {
    run_test_rom("individual/10-bit_ops.gb", 7_000_000);
}

#[test]
fn op_a_hl() {
    run_test_rom("individual/11-op_a_hl.gb", 3_000_000);
}

#[test]
fn cpu_instrs() {
    run_test_rom("cpu_instrs.gb", 3_000_000);
}

fn run_test_rom(test_rom: &str, max_cycles: u64) {
    let test_rom_path = Path::new("blargg_test_roms/cpu_instrs").join(test_rom);

    let mut emulator = Emulator::new(None, test_rom_path.to_str().unwrap());
    for _ in 0..max_cycles {
        emulator.tick()
    }

    let serial_data = emulator.get_serial_data();
    let serial_string = str::from_utf8(serial_data).unwrap();
    assert!(
        serial_string.find("Passed").is_some(),
        "Test rom failed with message '{}'",
        serial_string
    );
}
