// #![allow(dead_code)]
// use memory::Memory;
// use registers::Registers;
// 
// // If we have a cpu struct
// // the body of the xor method could be as simple as
// //     self.xor(self.load_u8(self.pc + 1), 2)
// // or even
// //     self.xor(self.load_u8_imm(), 2)
// 
// include!(concat!(env!("OUT_DIR"), "/generated_opcodes.rs"));
// 
// pub fn illegal_instruction(_: &mut Registers, _: &mut Memory) {
//     panic!("Illegal instruction!");
// }
// 
// pub fn prefix_cb(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction prefix_cb should not be called!");
// }
// 
// pub fn bit_check(regs: &mut Registers, value: u8) {
//     regs.f &= 0b0001_0000;
//     regs.set_flagh(true);
//     regs.set_flagz(value == 0);
// }
// 
// pub fn xor_check(regs: &mut Registers) {
//     regs.clear_flags();
//     let flagz = regs.a == 0;
//     regs.set_flagz(flagz);
// }
// 
// pub fn ld_32(regs: &mut Registers, mem: &mut Memory) {
//     let hl = regs.hld();
//     mem.set_u8(hl, regs.a);
//     regs.pc += 1;
// }
// 
// pub fn nop_00(regs: &mut Registers, _: &mut Memory) {
//     regs.pc += 1;
// }
// 
// pub fn jr_20(regs: &mut Registers, mem: &mut Memory) {
//     if !regs.flagz() {
//         let v = mem.get_u8(regs.pc + 1);
//         regs.pc += 2;
//         if v & 0b1000_0000 != 0 {
//             regs.pc -= (!v + 1) as u16;
//         } else {
//             regs.pc += v as u16;
//         }
//     } else {
//         regs.pc += 2;
//     }
// }
// 
// pub fn ld_02(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_02 is not implemented");
// }
// 
// pub fn inc_03(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_03 is not implemented");
// }
// 
// pub fn inc_04(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_04 is not implemented");
// }
// pub fn dec_05(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_05 is not implemented");
// }
// pub fn rlca_07(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rlca_07 is not implemented");
// }
// pub fn ld_08(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_08 is not implemented");
// }
// pub fn add_09(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_09 is not implemented");
// }
// pub fn ld_0a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_0a is not implemented");
// }
// pub fn dec_0b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_0b is not implemented");
// }
// pub fn inc_0c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_0c is not implemented");
// }
// pub fn dec_0d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_0d is not implemented");
// }
// pub fn rrca_0f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rrca_0f is not implemented");
// }
// pub fn stop_10(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction stop_10 is not implemented");
// }
// pub fn ld_12(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_12 is not implemented");
// }
// pub fn inc_13(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_13 is not implemented");
// }
// pub fn inc_14(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_14 is not implemented");
// }
// pub fn dec_15(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_15 is not implemented");
// }
// pub fn rla_17(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rla_17 is not implemented");
// }
// pub fn jr_18(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction jr_18 is not implemented");
// }
// pub fn add_19(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_19 is not implemented");
// }
// pub fn ld_1a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_1a is not implemented");
// }
// pub fn dec_1b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_1b is not implemented");
// }
// pub fn inc_1c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_1c is not implemented");
// }
// pub fn dec_1d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_1d is not implemented");
// }
// pub fn rra_1f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rra_1f is not implemented");
// }
// pub fn ld_22(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_22 is not implemented");
// }
// pub fn inc_23(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_23 is not implemented");
// }
// pub fn inc_24(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_24 is not implemented");
// }
// pub fn dec_25(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_25 is not implemented");
// }
// pub fn daa_27(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction daa_27 is not implemented");
// }
// pub fn jr_28(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction jr_28 is not implemented");
// }
// pub fn add_29(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_29 is not implemented");
// }
// pub fn ld_2a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_2a is not implemented");
// }
// pub fn dec_2b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_2b is not implemented");
// }
// pub fn inc_2c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_2c is not implemented");
// }
// pub fn dec_2d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_2d is not implemented");
// }
// pub fn cpl_2f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction cpl_2f is not implemented");
// }
// pub fn jr_30(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction jr_30 is not implemented");
// }
// pub fn inc_33(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_33 is not implemented");
// }
// pub fn inc_34(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_34 is not implemented");
// }
// pub fn dec_35(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_35 is not implemented");
// }
// pub fn ld_36(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_36 is not implemented");
// }
// pub fn scf_37(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction scf_37 is not implemented");
// }
// pub fn jr_38(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction jr_38 is not implemented");
// }
// pub fn add_39(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_39 is not implemented");
// }
// pub fn ld_3a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_3a is not implemented");
// }
// pub fn dec_3b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_3b is not implemented");
// }
// pub fn inc_3c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction inc_3c is not implemented");
// }
// pub fn dec_3d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction dec_3d is not implemented");
// }
// pub fn ld_3e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_3e is not implemented");
// }
// pub fn ccf_3f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ccf_3f is not implemented");
// }
// pub fn ld_40(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_40 is not implemented");
// }
// pub fn ld_41(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_41 is not implemented");
// }
// pub fn ld_42(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_42 is not implemented");
// }
// pub fn ld_43(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_43 is not implemented");
// }
// pub fn ld_44(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_44 is not implemented");
// }
// pub fn ld_45(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_45 is not implemented");
// }
// pub fn ld_46(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_46 is not implemented");
// }
// pub fn ld_47(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_47 is not implemented");
// }
// pub fn ld_48(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_48 is not implemented");
// }
// pub fn ld_49(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_49 is not implemented");
// }
// pub fn ld_4a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_4a is not implemented");
// }
// pub fn ld_4b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_4b is not implemented");
// }
// pub fn ld_4c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_4c is not implemented");
// }
// pub fn ld_4d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_4d is not implemented");
// }
// pub fn ld_4e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_4e is not implemented");
// }
// pub fn ld_4f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_4f is not implemented");
// }
// pub fn ld_50(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_50 is not implemented");
// }
// pub fn ld_51(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_51 is not implemented");
// }
// pub fn ld_52(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_52 is not implemented");
// }
// pub fn ld_53(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_53 is not implemented");
// }
// pub fn ld_54(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_54 is not implemented");
// }
// pub fn ld_55(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_55 is not implemented");
// }
// pub fn ld_56(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_56 is not implemented");
// }
// pub fn ld_57(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_57 is not implemented");
// }
// pub fn ld_58(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_58 is not implemented");
// }
// pub fn ld_59(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_59 is not implemented");
// }
// pub fn ld_5a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_5a is not implemented");
// }
// pub fn ld_5b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_5b is not implemented");
// }
// pub fn ld_5c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_5c is not implemented");
// }
// pub fn ld_5d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_5d is not implemented");
// }
// pub fn ld_5e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_5e is not implemented");
// }
// pub fn ld_5f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_5f is not implemented");
// }
// pub fn ld_60(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_60 is not implemented");
// }
// pub fn ld_61(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_61 is not implemented");
// }
// pub fn ld_62(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_62 is not implemented");
// }
// pub fn ld_63(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_63 is not implemented");
// }
// pub fn ld_64(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_64 is not implemented");
// }
// pub fn ld_65(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_65 is not implemented");
// }
// pub fn ld_66(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_66 is not implemented");
// }
// pub fn ld_67(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_67 is not implemented");
// }
// pub fn ld_68(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_68 is not implemented");
// }
// pub fn ld_69(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_69 is not implemented");
// }
// pub fn ld_6a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_6a is not implemented");
// }
// pub fn ld_6b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_6b is not implemented");
// }
// pub fn ld_6c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_6c is not implemented");
// }
// pub fn ld_6d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_6d is not implemented");
// }
// pub fn ld_6e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_6e is not implemented");
// }
// pub fn ld_6f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_6f is not implemented");
// }
// pub fn ld_70(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_70 is not implemented");
// }
// pub fn ld_71(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_71 is not implemented");
// }
// pub fn ld_72(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_72 is not implemented");
// }
// pub fn ld_73(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_73 is not implemented");
// }
// pub fn ld_74(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_74 is not implemented");
// }
// pub fn ld_75(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_75 is not implemented");
// }
// pub fn halt_76(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction halt_76 is not implemented");
// }
// pub fn ld_77(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_77 is not implemented");
// }
// pub fn ld_78(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_78 is not implemented");
// }
// pub fn ld_79(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_79 is not implemented");
// }
// pub fn ld_7a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_7a is not implemented");
// }
// pub fn ld_7b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_7b is not implemented");
// }
// pub fn ld_7c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_7c is not implemented");
// }
// pub fn ld_7d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_7d is not implemented");
// }
// pub fn ld_7e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_7e is not implemented");
// }
// pub fn ld_7f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_7f is not implemented");
// }
// pub fn add_80(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_80 is not implemented");
// }
// pub fn add_81(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_81 is not implemented");
// }
// pub fn add_82(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_82 is not implemented");
// }
// pub fn add_83(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_83 is not implemented");
// }
// pub fn add_84(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_84 is not implemented");
// }
// pub fn add_85(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_85 is not implemented");
// }
// pub fn add_86(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_86 is not implemented");
// }
// pub fn add_87(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_87 is not implemented");
// }
// pub fn adc_88(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction adc_88 is not implemented");
// }
// pub fn adc_89(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction adc_89 is not implemented");
// }
// pub fn adc_8a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction adc_8a is not implemented");
// }
// pub fn adc_8b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction adc_8b is not implemented");
// }
// pub fn adc_8c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction adc_8c is not implemented");
// }
// pub fn adc_8d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction adc_8d is not implemented");
// }
// pub fn adc_8e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction adc_8e is not implemented");
// }
// pub fn adc_8f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction adc_8f is not implemented");
// }
// pub fn sub_90(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sub_90 is not implemented");
// }
// pub fn sub_91(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sub_91 is not implemented");
// }
// pub fn sub_92(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sub_92 is not implemented");
// }
// pub fn sub_93(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sub_93 is not implemented");
// }
// pub fn sub_94(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sub_94 is not implemented");
// }
// pub fn sub_95(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sub_95 is not implemented");
// }
// pub fn sub_96(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sub_96 is not implemented");
// }
// pub fn sub_97(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sub_97 is not implemented");
// }
// pub fn sbc_98(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sbc_98 is not implemented");
// }
// pub fn sbc_99(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sbc_99 is not implemented");
// }
// pub fn sbc_9a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sbc_9a is not implemented");
// }
// pub fn sbc_9b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sbc_9b is not implemented");
// }
// pub fn sbc_9c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sbc_9c is not implemented");
// }
// pub fn sbc_9d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sbc_9d is not implemented");
// }
// pub fn sbc_9e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sbc_9e is not implemented");
// }
// pub fn sbc_9f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sbc_9f is not implemented");
// }
// pub fn and_a0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction and_a0 is not implemented");
// }
// pub fn and_a1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction and_a1 is not implemented");
// }
// pub fn and_a2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction and_a2 is not implemented");
// }
// pub fn and_a3(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction and_a3 is not implemented");
// }
// pub fn and_a4(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction and_a4 is not implemented");
// }
// pub fn and_a5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction and_a5 is not implemented");
// }
// pub fn and_a6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction and_a6 is not implemented");
// }
// pub fn and_a7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction and_a7 is not implemented");
// }
// pub fn or_b0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction or_b0 is not implemented");
// }
// pub fn or_b1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction or_b1 is not implemented");
// }
// pub fn or_b2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction or_b2 is not implemented");
// }
// pub fn or_b3(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction or_b3 is not implemented");
// }
// pub fn or_b4(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction or_b4 is not implemented");
// }
// pub fn or_b5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction or_b5 is not implemented");
// }
// pub fn or_b6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction or_b6 is not implemented");
// }
// pub fn or_b7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction or_b7 is not implemented");
// }
// pub fn cp_b8(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction cp_b8 is not implemented");
// }
// pub fn cp_b9(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction cp_b9 is not implemented");
// }
// pub fn cp_ba(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction cp_ba is not implemented");
// }
// pub fn cp_bb(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction cp_bb is not implemented");
// }
// pub fn cp_bc(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction cp_bc is not implemented");
// }
// pub fn cp_bd(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction cp_bd is not implemented");
// }
// pub fn cp_be(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction cp_be is not implemented");
// }
// pub fn cp_bf(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction cp_bf is not implemented");
// }
// pub fn ret_c0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ret_c0 is not implemented");
// }
// pub fn pop_c1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction pop_c1 is not implemented");
// }
// pub fn jp_c2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction jp_c2 is not implemented");
// }
// pub fn jp_c3(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction jp_c3 is not implemented");
// }
// pub fn call_c4(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction call_c4 is not implemented");
// }
// pub fn push_c5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction push_c5 is not implemented");
// }
// pub fn add_c6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_c6 is not implemented");
// }
// pub fn rst_c7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rst_c7 is not implemented");
// }
// pub fn ret_c8(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ret_c8 is not implemented");
// }
// pub fn ret_c9(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ret_c9 is not implemented");
// }
// pub fn jp_ca(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction jp_ca is not implemented");
// }
// pub fn call_cc(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction call_cc is not implemented");
// }
// pub fn call_cd(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction call_cd is not implemented");
// }
// pub fn adc_ce(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction adc_ce is not implemented");
// }
// pub fn rst_cf(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rst_cf is not implemented");
// }
// pub fn ret_d0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ret_d0 is not implemented");
// }
// pub fn pop_d1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction pop_d1 is not implemented");
// }
// pub fn jp_d2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction jp_d2 is not implemented");
// }
// pub fn call_d4(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction call_d4 is not implemented");
// }
// pub fn push_d5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction push_d5 is not implemented");
// }
// pub fn sub_d6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sub_d6 is not implemented");
// }
// pub fn rst_d7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rst_d7 is not implemented");
// }
// pub fn ret_d8(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ret_d8 is not implemented");
// }
// pub fn reti_d9(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction reti_d9 is not implemented");
// }
// pub fn jp_da(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction jp_da is not implemented");
// }
// pub fn call_dc(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction call_dc is not implemented");
// }
// pub fn sbc_de(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sbc_de is not implemented");
// }
// pub fn rst_df(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rst_df is not implemented");
// }
// pub fn ld_e0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_e0 is not implemented");
// }
// pub fn pop_e1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction pop_e1 is not implemented");
// }
// pub fn ld_e2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_e2 is not implemented");
// }
// pub fn push_e5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction push_e5 is not implemented");
// }
// pub fn and_e6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction and_e6 is not implemented");
// }
// pub fn rst_e7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rst_e7 is not implemented");
// }
// pub fn add_e8(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction add_e8 is not implemented");
// }
// pub fn jp_e9(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction jp_e9 is not implemented");
// }
// pub fn ld_ea(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_ea is not implemented");
// }
// pub fn rst_ef(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rst_ef is not implemented");
// }
// pub fn ld_f0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_f0 is not implemented");
// }
// pub fn pop_f1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction pop_f1 is not implemented");
// }
// pub fn ld_f2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_f2 is not implemented");
// }
// pub fn di_f3(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction di_f3 is not implemented");
// }
// pub fn push_f5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction push_f5 is not implemented");
// }
// pub fn or_f6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction or_f6 is not implemented");
// }
// pub fn rst_f7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rst_f7 is not implemented");
// }
// pub fn ld_f8(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_f8 is not implemented");
// }
// pub fn ld_f9(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_f9 is not implemented");
// }
// pub fn ld_fa(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ld_fa is not implemented");
// }
// pub fn ei_fb(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction ei_fb is not implemented");
// }
// pub fn cp_fe(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction cp_fe is not implemented");
// }
// pub fn rst_ff(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rst_ff is not implemented");
// }
// pub fn rlc_100(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rlc_100 is not implemented");
// }
// pub fn rlc_101(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rlc_101 is not implemented");
// }
// pub fn rlc_102(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rlc_102 is not implemented");
// }
// pub fn rlc_103(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rlc_103 is not implemented");
// }
// pub fn rlc_104(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rlc_104 is not implemented");
// }
// pub fn rlc_105(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rlc_105 is not implemented");
// }
// pub fn rlc_106(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rlc_106 is not implemented");
// }
// pub fn rlc_107(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rlc_107 is not implemented");
// }
// pub fn rrc_108(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rrc_108 is not implemented");
// }
// pub fn rrc_109(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rrc_109 is not implemented");
// }
// pub fn rrc_10a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rrc_10a is not implemented");
// }
// pub fn rrc_10b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rrc_10b is not implemented");
// }
// pub fn rrc_10c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rrc_10c is not implemented");
// }
// pub fn rrc_10d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rrc_10d is not implemented");
// }
// pub fn rrc_10e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rrc_10e is not implemented");
// }
// pub fn rrc_10f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rrc_10f is not implemented");
// }
// pub fn rl_110(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rl_110 is not implemented");
// }
// pub fn rl_111(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rl_111 is not implemented");
// }
// pub fn rl_112(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rl_112 is not implemented");
// }
// pub fn rl_113(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rl_113 is not implemented");
// }
// pub fn rl_114(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rl_114 is not implemented");
// }
// pub fn rl_115(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rl_115 is not implemented");
// }
// pub fn rl_116(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rl_116 is not implemented");
// }
// pub fn rl_117(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rl_117 is not implemented");
// }
// pub fn rr_118(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rr_118 is not implemented");
// }
// pub fn rr_119(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rr_119 is not implemented");
// }
// pub fn rr_11a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rr_11a is not implemented");
// }
// pub fn rr_11b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rr_11b is not implemented");
// }
// pub fn rr_11c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rr_11c is not implemented");
// }
// pub fn rr_11d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rr_11d is not implemented");
// }
// pub fn rr_11e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rr_11e is not implemented");
// }
// pub fn rr_11f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction rr_11f is not implemented");
// }
// pub fn sla_120(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sla_120 is not implemented");
// }
// pub fn sla_121(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sla_121 is not implemented");
// }
// pub fn sla_122(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sla_122 is not implemented");
// }
// pub fn sla_123(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sla_123 is not implemented");
// }
// pub fn sla_124(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sla_124 is not implemented");
// }
// pub fn sla_125(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sla_125 is not implemented");
// }
// pub fn sla_126(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sla_126 is not implemented");
// }
// pub fn sla_127(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sla_127 is not implemented");
// }
// pub fn sra_128(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sra_128 is not implemented");
// }
// pub fn sra_129(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sra_129 is not implemented");
// }
// pub fn sra_12a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sra_12a is not implemented");
// }
// pub fn sra_12b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sra_12b is not implemented");
// }
// pub fn sra_12c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sra_12c is not implemented");
// }
// pub fn sra_12d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sra_12d is not implemented");
// }
// pub fn sra_12e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sra_12e is not implemented");
// }
// pub fn sra_12f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction sra_12f is not implemented");
// }
// pub fn swap_130(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction swap_130 is not implemented");
// }
// pub fn swap_131(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction swap_131 is not implemented");
// }
// pub fn swap_132(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction swap_132 is not implemented");
// }
// pub fn swap_133(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction swap_133 is not implemented");
// }
// pub fn swap_134(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction swap_134 is not implemented");
// }
// pub fn swap_135(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction swap_135 is not implemented");
// }
// pub fn swap_136(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction swap_136 is not implemented");
// }
// pub fn swap_137(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction swap_137 is not implemented");
// }
// pub fn srl_138(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction srl_138 is not implemented");
// }
// pub fn srl_139(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction srl_139 is not implemented");
// }
// pub fn srl_13a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction srl_13a is not implemented");
// }
// pub fn srl_13b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction srl_13b is not implemented");
// }
// pub fn srl_13c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction srl_13c is not implemented");
// }
// pub fn srl_13d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction srl_13d is not implemented");
// }
// pub fn srl_13e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction srl_13e is not implemented");
// }
// pub fn srl_13f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction srl_13f is not implemented");
// }
// pub fn res_180(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_180 is not implemented");
// }
// pub fn res_181(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_181 is not implemented");
// }
// pub fn res_182(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_182 is not implemented");
// }
// pub fn res_183(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_183 is not implemented");
// }
// pub fn res_184(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_184 is not implemented");
// }
// pub fn res_185(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_185 is not implemented");
// }
// pub fn res_186(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_186 is not implemented");
// }
// pub fn res_187(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_187 is not implemented");
// }
// pub fn res_188(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_188 is not implemented");
// }
// pub fn res_189(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_189 is not implemented");
// }
// pub fn res_18a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_18a is not implemented");
// }
// pub fn res_18b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_18b is not implemented");
// }
// pub fn res_18c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_18c is not implemented");
// }
// pub fn res_18d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_18d is not implemented");
// }
// pub fn res_18e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_18e is not implemented");
// }
// pub fn res_18f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_18f is not implemented");
// }
// pub fn res_190(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_190 is not implemented");
// }
// pub fn res_191(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_191 is not implemented");
// }
// pub fn res_192(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_192 is not implemented");
// }
// pub fn res_193(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_193 is not implemented");
// }
// pub fn res_194(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_194 is not implemented");
// }
// pub fn res_195(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_195 is not implemented");
// }
// pub fn res_196(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_196 is not implemented");
// }
// pub fn res_197(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_197 is not implemented");
// }
// pub fn res_198(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_198 is not implemented");
// }
// pub fn res_199(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_199 is not implemented");
// }
// pub fn res_19a(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_19a is not implemented");
// }
// pub fn res_19b(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_19b is not implemented");
// }
// pub fn res_19c(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_19c is not implemented");
// }
// pub fn res_19d(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_19d is not implemented");
// }
// pub fn res_19e(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_19e is not implemented");
// }
// pub fn res_19f(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_19f is not implemented");
// }
// pub fn res_1a0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1a0 is not implemented");
// }
// pub fn res_1a1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1a1 is not implemented");
// }
// pub fn res_1a2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1a2 is not implemented");
// }
// pub fn res_1a3(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1a3 is not implemented");
// }
// pub fn res_1a4(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1a4 is not implemented");
// }
// pub fn res_1a5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1a5 is not implemented");
// }
// pub fn res_1a6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1a6 is not implemented");
// }
// pub fn res_1a7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1a7 is not implemented");
// }
// pub fn res_1a8(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1a8 is not implemented");
// }
// pub fn res_1a9(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1a9 is not implemented");
// }
// pub fn res_1aa(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1aa is not implemented");
// }
// pub fn res_1ab(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1ab is not implemented");
// }
// pub fn res_1ac(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1ac is not implemented");
// }
// pub fn res_1ad(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1ad is not implemented");
// }
// pub fn res_1ae(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1ae is not implemented");
// }
// pub fn res_1af(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1af is not implemented");
// }
// pub fn res_1b0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1b0 is not implemented");
// }
// pub fn res_1b1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1b1 is not implemented");
// }
// pub fn res_1b2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1b2 is not implemented");
// }
// pub fn res_1b3(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1b3 is not implemented");
// }
// pub fn res_1b4(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1b4 is not implemented");
// }
// pub fn res_1b5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1b5 is not implemented");
// }
// pub fn res_1b6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1b6 is not implemented");
// }
// pub fn res_1b7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1b7 is not implemented");
// }
// pub fn res_1b8(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1b8 is not implemented");
// }
// pub fn res_1b9(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1b9 is not implemented");
// }
// pub fn res_1ba(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1ba is not implemented");
// }
// pub fn res_1bb(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1bb is not implemented");
// }
// pub fn res_1bc(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1bc is not implemented");
// }
// pub fn res_1bd(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1bd is not implemented");
// }
// pub fn res_1be(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1be is not implemented");
// }
// pub fn res_1bf(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction res_1bf is not implemented");
// }
// pub fn set_1c0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1c0 is not implemented");
// }
// pub fn set_1c1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1c1 is not implemented");
// }
// pub fn set_1c2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1c2 is not implemented");
// }
// pub fn set_1c3(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1c3 is not implemented");
// }
// pub fn set_1c4(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1c4 is not implemented");
// }
// pub fn set_1c5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1c5 is not implemented");
// }
// pub fn set_1c6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1c6 is not implemented");
// }
// pub fn set_1c7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1c7 is not implemented");
// }
// pub fn set_1c8(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1c8 is not implemented");
// }
// pub fn set_1c9(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1c9 is not implemented");
// }
// pub fn set_1ca(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1ca is not implemented");
// }
// pub fn set_1cb(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1cb is not implemented");
// }
// pub fn set_1cc(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1cc is not implemented");
// }
// pub fn set_1cd(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1cd is not implemented");
// }
// pub fn set_1ce(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1ce is not implemented");
// }
// pub fn set_1cf(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1cf is not implemented");
// }
// pub fn set_1d0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1d0 is not implemented");
// }
// pub fn set_1d1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1d1 is not implemented");
// }
// pub fn set_1d2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1d2 is not implemented");
// }
// pub fn set_1d3(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1d3 is not implemented");
// }
// pub fn set_1d4(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1d4 is not implemented");
// }
// pub fn set_1d5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1d5 is not implemented");
// }
// pub fn set_1d6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1d6 is not implemented");
// }
// pub fn set_1d7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1d7 is not implemented");
// }
// pub fn set_1d8(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1d8 is not implemented");
// }
// pub fn set_1d9(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1d9 is not implemented");
// }
// pub fn set_1da(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1da is not implemented");
// }
// pub fn set_1db(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1db is not implemented");
// }
// pub fn set_1dc(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1dc is not implemented");
// }
// pub fn set_1dd(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1dd is not implemented");
// }
// pub fn set_1de(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1de is not implemented");
// }
// pub fn set_1df(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1df is not implemented");
// }
// pub fn set_1e0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1e0 is not implemented");
// }
// pub fn set_1e1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1e1 is not implemented");
// }
// pub fn set_1e2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1e2 is not implemented");
// }
// pub fn set_1e3(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1e3 is not implemented");
// }
// pub fn set_1e4(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1e4 is not implemented");
// }
// pub fn set_1e5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1e5 is not implemented");
// }
// pub fn set_1e6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1e6 is not implemented");
// }
// pub fn set_1e7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1e7 is not implemented");
// }
// pub fn set_1e8(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1e8 is not implemented");
// }
// pub fn set_1e9(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1e9 is not implemented");
// }
// pub fn set_1ea(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1ea is not implemented");
// }
// pub fn set_1eb(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1eb is not implemented");
// }
// pub fn set_1ec(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1ec is not implemented");
// }
// pub fn set_1ed(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1ed is not implemented");
// }
// pub fn set_1ee(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1ee is not implemented");
// }
// pub fn set_1ef(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1ef is not implemented");
// }
// pub fn set_1f0(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1f0 is not implemented");
// }
// pub fn set_1f1(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1f1 is not implemented");
// }
// pub fn set_1f2(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1f2 is not implemented");
// }
// pub fn set_1f3(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1f3 is not implemented");
// }
// pub fn set_1f4(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1f4 is not implemented");
// }
// pub fn set_1f5(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1f5 is not implemented");
// }
// pub fn set_1f6(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1f6 is not implemented");
// }
// pub fn set_1f7(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1f7 is not implemented");
// }
// pub fn set_1f8(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1f8 is not implemented");
// }
// pub fn set_1f9(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1f9 is not implemented");
// }
// pub fn set_1fa(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1fa is not implemented");
// }
// pub fn set_1fb(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1fb is not implemented");
// }
// pub fn set_1fc(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1fc is not implemented");
// }
// pub fn set_1fd(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1fd is not implemented");
// }
// pub fn set_1fe(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1fe is not implemented");
// }
// pub fn set_1ff(_: &mut Registers, _: &mut Memory) {
//     panic!("Instruction set_1ff is not implemented");
// }
