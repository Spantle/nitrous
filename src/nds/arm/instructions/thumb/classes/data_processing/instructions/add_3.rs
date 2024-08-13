use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// ADD (3)
pub fn add_3(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("ADD");

    let rd = ctx.inst.get_byte(0, 2);
    let rn = ctx.inst.get_byte(3, 5);
    let rm = ctx.inst.get_byte(6, 8);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rn, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rm, None);

    let rn = ctx.arm.r()[rn];
    let rm = ctx.arm.r()[rm];
    let (result, carry) = rn.overflowing_add(rm);
    ctx.arm.set_r(rd, result);
    ctx.arm.cpsr_mut().set_negative(result.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(result == 0);
    ctx.arm.cpsr_mut().set_carry(carry);
    ctx.arm
        .cpsr_mut()
        .set_overflow((rn as i32).overflowing_add(rm as i32).1);

    1 // TODO: this is wrong
}
