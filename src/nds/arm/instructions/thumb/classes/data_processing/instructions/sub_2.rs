use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// SUB (2)
pub fn sub_2(ctx: &mut Context<Instruction, impl ContextTrait>) {
    ctx.dis.set_inst("SUB");

    let rd = ctx.inst.get_byte(8, 10);
    let immed_8 = ctx.inst.get_word(0, 7);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_word_arg(immed_8);

    let (result, borrow) = ctx.arm.r()[rd].overflowing_sub(immed_8);
    ctx.arm.set_r(rd, result);
    ctx.arm.cpsr_mut().set_negative(result.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(result == 0);
    ctx.arm.cpsr_mut().set_carry(!borrow);
    ctx.arm
        .cpsr_mut()
        .set_overflow((result as i32).overflowing_sub(immed_8 as i32).1);
}
