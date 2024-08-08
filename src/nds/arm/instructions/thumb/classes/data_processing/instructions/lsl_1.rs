use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// LSL (1)
pub fn lsl_1(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("LSL");

    let rd = ctx.inst.get_byte(0, 2);
    let rm = ctx.inst.get_byte(3, 5);
    let immed_5 = ctx.inst.get_word(6, 10);
    ctx.dis.push_reg_arg(rd, Some(", "));
    ctx.dis.push_reg_arg(rm, None);
    ctx.dis.push_word_end_arg(immed_5, None);

    if immed_5 == 0 {
        let rm = ctx.arm.r()[rm];
        ctx.arm.set_r(rd, rm);
    } else {
        let rm = ctx.arm.r()[rm];
        ctx.arm.set_r(rd, rm << immed_5);

        let rd = ctx.arm.r()[rd];
        ctx.arm.cpsr_mut().set_carry(rd.get_bit(32 - immed_5));
    }

    let rd = ctx.arm.r()[rd];
    ctx.arm.cpsr_mut().set_negative(rd.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(rd == 0);

    1
}
