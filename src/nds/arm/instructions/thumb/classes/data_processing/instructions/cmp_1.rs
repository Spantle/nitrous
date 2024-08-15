use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// CMP (1)
pub fn cmp_1(ctx: &mut Context<Instruction, impl ContextTrait>) {
    ctx.dis.set_inst("CMP");

    let rn = ctx.inst.get_byte(8, 10);
    let immed_8 = ctx.inst.get_word(0, 7);
    ctx.dis.push_reg_arg(rn, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_word_arg(immed_8);

    let (alu_out, borrow) = ctx.arm.r()[rn].overflowing_sub(immed_8);
    let overflow = (ctx.arm.r()[rn] as i32).overflowing_sub(immed_8 as i32).1;
    let cpsr = ctx.arm.cpsr_mut();
    cpsr.set_negative(alu_out.get_bit(31));
    cpsr.set_zero(alu_out == 0);
    cpsr.set_carry(!borrow);
    cpsr.set_overflow(overflow);
}
