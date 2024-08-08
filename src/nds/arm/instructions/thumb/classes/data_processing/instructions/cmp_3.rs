use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// CMP (3)
pub fn cmp_3(ctx: &mut Context<Instruction, impl ContextTrait>) {
    ctx.dis.set_inst("CMP");

    let rn = ctx.inst.get_byte(0, 2);
    let rm = ctx.inst.get_byte(3, 5);
    let h1 = ctx.inst.get_bit(6);
    let h2 = ctx.inst.get_bit(7);
    let rn = ctx.inst.get_rh(h1, rn);
    let rm = ctx.inst.get_rh(h2, rm);
    ctx.dis.push_reg_arg(rn, Some(", "));
    ctx.dis.push_reg_arg(rm, None);

    let (alu_out, borrow) = ctx.arm.ert(rn).overflowing_sub(ctx.arm.ert(rm));
    let overflow = (ctx.arm.ert(rn) as i32)
        .overflowing_sub(ctx.arm.ert(rm) as i32)
        .1;
    let cpsr = ctx.arm.cpsr_mut();
    cpsr.set_negative(alu_out.get_bit(31));
    cpsr.set_zero(alu_out == 0);
    cpsr.set_carry(!borrow);
    cpsr.set_overflow(overflow);
}
