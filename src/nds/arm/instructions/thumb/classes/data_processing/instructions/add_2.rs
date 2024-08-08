use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// ADD (2)
pub fn add_2(ctx: &mut Context<Instruction, impl ContextTrait>) {
    ctx.dis.set_inst("ADD");

    let rd = ctx.inst.get_byte(8, 10);
    let immed_8 = ctx.inst.get_word(0, 7);
    ctx.dis.push_reg_arg(rd, Some(", "));
    ctx.dis.push_word_end_arg(immed_8, None);

    let (result, carry) = ctx.arm.r()[rd].overflowing_add(immed_8);

    ctx.arm.set_r(rd, result);

    let rd = ctx.arm.r()[rd];
    ctx.arm.cpsr_mut().set_negative(result.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(result == 0);
    ctx.arm.cpsr_mut().set_carry(carry);
    ctx.arm
        .cpsr_mut()
        .set_overflow((rd as i32).overflowing_add(immed_8 as i32).1);
}
