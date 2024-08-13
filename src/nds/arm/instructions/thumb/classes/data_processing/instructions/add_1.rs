use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// ADD (1)
pub fn add_1(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("ADD");

    let rd = ctx.inst.get_byte(0, 2);
    let rn = ctx.inst.get_byte(3, 5);
    let immed_3 = ctx.inst.get_word(6, 8);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rn, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_word_arg(immed_3);

    let rn = ctx.arm.r()[rn];
    let (result, carry) = rn.overflowing_add(immed_3);
    ctx.arm.set_r(rd, result);
    ctx.arm.cpsr_mut().set_negative(result.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(result == 0);
    ctx.arm.cpsr_mut().set_carry(carry);
    ctx.arm
        .cpsr_mut()
        .set_overflow((rn as i32).overflowing_add(immed_3 as i32).1);

    1 // TODO: this is wrong
}
