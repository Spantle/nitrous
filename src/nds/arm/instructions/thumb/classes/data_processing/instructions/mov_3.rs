use crate::nds::arm::{
    arm::ArmTrait,
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
};

// MOV (3)
pub fn mov_3(ctx: &mut Context<Instruction, impl ContextTrait>) {
    ctx.dis.set_inst("MOV");

    let rd = ctx.inst.get_byte(0, 2);
    let rm = ctx.inst.get_byte(3, 5);
    let h1 = ctx.inst.get_bit(7);
    let h2 = ctx.inst.get_bit(6);
    let rd = ctx.inst.get_rh(h1, rd);
    let rm = ctx.inst.get_rh(h2, rm);
    ctx.dis.push_reg_arg(rd, Some(", "));
    ctx.dis.push_reg_arg(rm, None);

    ctx.arm.set_r(rd, ctx.arm.r()[rm]);
}
