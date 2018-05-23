extern crate strfmt;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use strfmt::strfmt;

struct FunctionGenerator<'a> {
    output_file: File,
    vars: HashMap<String, &'a str>,
}

impl<'a> FunctionGenerator<'a> {
    fn new(fname: &str) -> FunctionGenerator {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join(fname);
        let output_file = File::create(&dest_path).unwrap();
        let vars = HashMap::new();

        FunctionGenerator { output_file, vars }
    }

    fn add_var(&mut self, key: &str, value: &'a str) {
        self.vars.insert(key.to_string(), value);
    }

    fn create_ld_r_n(&mut self, opcode: &str, reg: &str) {
        let template = "pub fn ld_{opcode}({args}) {{
    regs.{reg} = _mem.get_u8(regs.pc + 1);
    regs.pc += 2;
}}

";
        let mut vars = self.vars.clone();
        vars.insert("opcode".to_string(), opcode);
        vars.insert("reg".to_string(), reg);
        let output = strfmt(&template, &vars).unwrap();
        self.output_file.write_all(output.as_bytes()).unwrap();
    }

    fn create_ld_rr_nn(&mut self, opcode: &str, reg: &str) {
        let template = "pub fn ld_{opcode}({args}) {{
    let v = _mem.get_u16(regs.pc + 1);
    regs.set_{reg}(v);
    regs.pc += 3;
}}

";
        let mut vars = self.vars.clone();
        vars.insert("opcode".to_string(), opcode);
        vars.insert("reg".to_string(), reg);
        let output = strfmt(&template, &vars).unwrap();
        self.output_file.write_all(output.as_bytes()).unwrap();
    }

    fn create_xor_r(&mut self, opcode: &str, n: &str, length: &str) {
        let template = "pub fn xor_{opcode}({args}) {{
    regs.a = regs.a ^ {n};
    xor_check(regs);
    regs.pc += {length};
}}

";
        let mut vars = self.vars.clone();
        vars.insert("opcode".to_string(), opcode);
        vars.insert("n".to_string(), n);
        vars.insert("length".to_string(), length);
        let output = strfmt(&template, &vars).unwrap();
        self.output_file.write_all(output.as_bytes()).unwrap();
    }

    fn create_bit_b_r(&mut self, opcode: &str, b: &str, r: &str) {
        let template = "pub fn bit_{opcode}({args}) {{
    let t = {r} & (1 << {b});
    bit_check(regs, t);
    regs.pc += 2;
}}

";
        let mut vars = self.vars.clone();
        vars.insert("opcode".to_string(), opcode);
        vars.insert("b".to_string(), b);
        vars.insert("r".to_string(), r);
        let output = strfmt(&template, &vars).unwrap();
        self.output_file.write_all(output.as_bytes()).unwrap();
    }
}

fn main() {
    let mut generator = FunctionGenerator::new("generated_opcodes.rs");
    generator.add_var("args", "regs: &mut Registers, _mem: &mut Memory");

    // 8 bit load immediate
    generator.create_ld_r_n("06", "b");
    generator.create_ld_r_n("0e", "c");
    generator.create_ld_r_n("16", "d");
    generator.create_ld_r_n("1e", "e");
    generator.create_ld_r_n("26", "h");
    generator.create_ld_r_n("2e", "l");

    // 16 bit load immediate
    generator.create_ld_rr_nn("01", "bc");
    generator.create_ld_rr_nn("11", "de");
    generator.create_ld_rr_nn("21", "hl");
    generator.create_ld_rr_nn("31", "sp");

    generator.create_xor_r("af", "regs.a", "1");
    generator.create_xor_r("a8", "regs.b", "1");
    generator.create_xor_r("a9", "regs.c", "1");
    generator.create_xor_r("aa", "regs.d", "1");
    generator.create_xor_r("ab", "regs.e", "1");
    generator.create_xor_r("ac", "regs.h", "1");
    generator.create_xor_r("ad", "regs.l", "1");
    generator.create_xor_r("ae", "_mem.get_u8(regs.get_hl())", "1");
    generator.create_xor_r("ee", "_mem.get_u8(regs.pc + 1)", "2");

    {
        // create all bit_b_r functions
        let srcs = [
            "regs.b",
            "regs.c",
            "regs.d",
            "regs.e",
            "regs.h",
            "regs.l",
            "_mem.get_u8(regs.get_hl())",
            "regs.a",
        ];

        let mut opc = 0x140;
        for shift in 0..8 {
            for &src in srcs.iter() {
                let opc_str = format!("{:x}", (opc));
                generator.create_bit_b_r(&opc_str, &shift.to_string(), src);
                opc += 1;
            }
        }
    }
}
