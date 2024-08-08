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
    let h1 = ctx.inst.get_bit(6);
    let h2 = ctx.inst.get_bit(7);
    ctx.dis.push_reg_arg(rd, Some(", "));
    ctx.dis.push_reg_arg(rm, None);

    let result = ctx.arm.ert(h1, rd).wrapping_add(ctx.arm.ert(h2, rm));
    ctx.arm.set_rt(h1, rd, result);
}
