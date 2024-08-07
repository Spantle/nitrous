use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// LSR (1)
pub fn lsr_1(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("LSR");

    let rd = ctx.inst.get_byte(0, 2);
    let rm = ctx.inst.get_byte(3, 5);
    let immed_5 = ctx.inst.get_word(6, 10);
    ctx.dis.push_reg_arg(rd, Some(", "));
    ctx.dis.push_reg_arg(rm, None);
    ctx.dis.push_word_end_arg(immed_5, None);

    let result = if immed_5 == 0 {
        let rd = ctx.arm.r()[rd];
        ctx.arm.cpsr_mut().set_carry(rd.get_bit(31));

        0
    } else {
        let rd = ctx.arm.r()[rd];
        ctx.arm.cpsr_mut().set_carry(rd.get_bit(immed_5 - 1));

        let rm = ctx.arm.r()[rm];
        rm >> immed_5
    };

    ctx.arm.set_r(rd, result);
    ctx.arm.cpsr_mut().set_negative(result.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(result == 0);

    1
}
