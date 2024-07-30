use crate::nds::arm::{
    arm::ArmTrait,
    models::{Context, ContextTrait, DisassemblyTrait, Instruction},
};

// CLZ
pub fn clz(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("CLZ");

    let rd = ctx.inst.get_byte(12, 15);
    let rn = ctx.inst.get_byte(0, 3);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rn, None);

    ctx.arm.set_r(rd, ctx.arm.er(rn).leading_zeros());

    1 // TODO: this is wrong
}
