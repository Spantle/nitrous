use crate::nds::arm::{
    instructions::arm::Instruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// MLA, MLAS
pub fn mla<const S: bool>(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let (inst, arm) = (&mut ctx.inst, &mut ctx.arm);
    let rm = inst.get_byte(0, 3);
    let rs = inst.get_byte(8, 11);
    let rn = inst.get_byte(12, 15);
    let rd = inst.get_byte(16, 19);

    ctx.dis.set_inst("MLA");
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rm, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rs, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rn, None);

    let result = arm.er(rm).wrapping_mul(arm.er(rs)).wrapping_add(arm.er(rn));
    arm.set_r(rd, result);
    if S {
        arm.cpsr_mut().set_negative(result.get_bit(31));
        arm.cpsr_mut().set_zero(result == 0);
    }

    1 // TODO: this is wrong
}
