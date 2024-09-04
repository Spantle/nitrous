use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// CMP (2)
pub fn cmp_2(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("CMP");

    let rn = ctx.inst.get_byte(0, 2);
    let rm = ctx.inst.get_byte(3, 5);
    ctx.dis.push_reg_arg(rn, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rm, None);

    let (rn, rm) = (ctx.arm.r()[rn], ctx.arm.r()[rm]);
    let (alu_out, borrow) = rn.overflowing_sub(rm);
    let overflow = (rn as i32).overflowing_sub(rm as i32).1;
    let cpsr = ctx.arm.cpsr_mut();
    cpsr.set_negative(alu_out.get_bit(31));
    cpsr.set_zero(alu_out == 0);
    cpsr.set_carry(!borrow);
    cpsr.set_overflow(overflow);

    1
}
