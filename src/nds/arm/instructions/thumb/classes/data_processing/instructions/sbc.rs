use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// SBC
pub fn sbc(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("SBC");

    let rd = ctx.inst.get_byte(0, 2);
    let rm = ctx.inst.get_byte(3, 5);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rm, None);

    let rm = ctx.arm.r()[rm];
    let c_flag = !ctx.arm.cpsr().get_carry() as u32;
    let (result, borrow1) = ctx.arm.r()[rd].overflowing_sub(rm);
    let (result, borrow2) = result.overflowing_sub(c_flag);
    let borrow = borrow1 || borrow2;

    ctx.arm.set_r(rd, result);
    ctx.arm.cpsr_mut().set_negative(result.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(result == 0);
    ctx.arm.cpsr_mut().set_carry(!borrow);

    let (result, overflow1) = (ctx.arm.r()[rd] as i32).overflowing_sub(rm as i32);
    let (_, overflow2) = result.overflowing_sub(c_flag as i32);
    ctx.arm.cpsr_mut().set_overflow(overflow1 || overflow2);

    1 // TODO: this is wrong
}
