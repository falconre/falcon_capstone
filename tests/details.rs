extern crate falcon_capstone;

use falcon_capstone::capstone as cs;

#[test]
fn implicit() {
    let code = vec![0x01, 0xdd, 0xe8, 0x06, 0x00, 0x00, 0x00];
    let dec = cs::Capstone::new(cs::cs_arch::CS_ARCH_X86, cs::CS_MODE_32).unwrap();
    dec.option(cs::cs_opt_type::CS_OPT_DETAIL, cs::cs_opt_value::CS_OPT_ON)
        .unwrap();

    let buf = dec.disasm(code.as_slice(), 0, 0).unwrap();

    let detail = buf.get(0).unwrap().detail.unwrap();
    assert_eq!(dec.reg_name(detail.regs_write[0]), Some("eflags"));

    let detail = buf.get(1).unwrap().detail.unwrap();
    assert_eq!(dec.reg_name(detail.regs_read[0]), Some("esp"));
}

#[test]
fn operands() {
    let code = vec![0x2b, 0x72, 0x05];
    let dec = cs::Capstone::new(cs::cs_arch::CS_ARCH_X86, cs::CS_MODE_32).unwrap();
    dec.option(cs::cs_opt_type::CS_OPT_DETAIL, cs::cs_opt_value::CS_OPT_ON)
        .unwrap();

    let buf = dec.disasm(code.as_slice(), 0, 0).unwrap();
    let detail = buf.get(0).unwrap().detail.unwrap();
    if let cs::DetailsArch::X86(arch) = detail.arch {
        let op1 = arch.operands[0];
        assert_eq!(op1.type_, cs::x86_op_type::X86_OP_REG);
        assert_eq!(dec.reg_name(op1.reg().as_int()), Some("esi"));

        let op2 = arch.operands[1];
        let mem = op2.mem();
        assert_eq!(op2.type_, cs::x86_op_type::X86_OP_MEM.into());
        assert_eq!(mem.segment, cs::x86_reg::X86_REG_INVALID.into());
        assert_eq!(mem.base, cs::x86_reg::X86_REG_EDX.into());
        assert_eq!(mem.index, cs::x86_reg::X86_REG_INVALID.into());
        assert_eq!(mem.scale, 1);
        assert_eq!(mem.disp, 5);
    }
}
