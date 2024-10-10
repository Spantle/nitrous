use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// CMN
pub fn cmn(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("CMN");

    let rn = ctx.inst.get_byte(0, 2);
    let rm = ctx.inst.get_byte(3, 5);
    ctx.dis.push_reg_arg(rn, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rm, None);

    let (result, borrow) = ctx.arm.r()[rn].overflowing_add(ctx.arm.r()[rm]);
    ctx.arm.cpsr_mut().set_negative(result.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(result == 0);
    ctx.arm.cpsr_mut().set_carry(!borrow);
    ctx.arm
        .cpsr_mut()
        .set_overflow((result as i32).overflowing_add(rm as i32).1);

    1 // TODO: this is wrong
}
