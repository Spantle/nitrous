use crate::nds::arm::{
    arm::ArmTrait,
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
};

// ADD (4)
pub fn add_4(ctx: &mut Context<Instruction, impl ContextTrait>) {
    ctx.dis.set_inst("ADD");

    let rd = ctx.inst.get_byte(0, 2);
    let rm = ctx.inst.get_byte(3, 5);
    let h1 = ctx.inst.get_bit(7);
    let h2 = ctx.inst.get_bit(6);
    let rd = ctx.inst.get_rh(h1, rd);
    let rm = ctx.inst.get_rh(h2, rm);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rm, None);

    let result = ctx.arm.ert(rd).wrapping_add(ctx.arm.ert(rm));
    ctx.arm.set_r(rd, result);
}
