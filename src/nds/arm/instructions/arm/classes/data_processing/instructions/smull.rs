use crate::nds::arm::{
    arm::ArmTrait,
    instructions::arm::Instruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
};

// SMULL, SMULLS
pub fn smull<const S: bool>(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let (inst, arm) = (&mut ctx.inst, &mut ctx.arm);
    let rm = inst.get_byte(0, 3);
    let rs = inst.get_byte(8, 11);
    let rd_lo = inst.get_byte(12, 15);
    let rd_hi = inst.get_byte(16, 19);

    ctx.dis.set_inst("SMULL");
    ctx.dis.push_reg_arg(rd_lo, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rd_hi, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rm, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rs, None);

    let result = (arm.er(rm) as i32 as i64).wrapping_mul(arm.er(rs) as i32 as i64) as u64;
    arm.set_r(rd_hi, result.get_bits(32, 63) as u32);
    arm.set_r(rd_lo, result.get_bits(0, 31) as u32);
    if S {
        arm.cpsr_mut()
            .set_negative(result.get_bit(63) || result.get_bit(31));
        arm.cpsr_mut().set_zero(result == 0);
    }

    1 // TODO: this is wrong
}
