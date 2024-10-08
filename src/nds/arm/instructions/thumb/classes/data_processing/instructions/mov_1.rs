use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// MOV (1)
pub fn mov_1(ctx: &mut Context<Instruction, impl ContextTrait>) {
    ctx.dis.set_inst("MOV");

    let rd = ctx.inst.get_byte(8, 10);
    let immed_8 = ctx.inst.get_word(0, 7);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_word_end_arg(immed_8, None);

    ctx.arm.set_r(rd, immed_8);
    ctx.arm.cpsr_mut().set_negative(immed_8.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(immed_8 == 0);
}
