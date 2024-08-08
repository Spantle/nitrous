use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// TST
pub fn tst(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("TST");

    let rn = ctx.inst.get_byte(0, 2);
    let rm = ctx.inst.get_byte(3, 5);
    ctx.dis.push_reg_arg(rn, Some(", "));
    ctx.dis.push_reg_arg(rm, None);

    let result = ctx.arm.r()[rn] & ctx.arm.r()[rm];
    ctx.arm.cpsr_mut().set_negative(result.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(result == 0);

    1 // TODO: this is wrong
}
