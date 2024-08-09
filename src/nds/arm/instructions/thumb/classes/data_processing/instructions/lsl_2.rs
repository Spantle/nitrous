use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// LSL (2)
pub fn lsl_2(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("LSL");

    let rd = ctx.inst.get_byte(0, 2);
    let rs = ctx.inst.get_byte(3, 5);
    ctx.dis.push_reg_arg(rd, Some(", "));
    ctx.dis.push_reg_arg(rs, None);

    let rs = ctx.arm.r()[rs];
    let byte = rs.get_bits(0, 7);
    let result = if byte == 0 {
        ctx.arm.r()[rd]
    } else if byte < 32 {
        let rd = ctx.arm.r()[rd];
        ctx.arm.cpsr_mut().set_carry(rd.get_bit(32 - byte));
        rd << byte
    } else if byte == 32 {
        let rd = ctx.arm.r()[rd];
        ctx.arm.cpsr_mut().set_carry(rd.get_bit(0));
        0
    } else {
        ctx.arm.cpsr_mut().set_carry(false);
        0
    };

    ctx.arm.set_r(rd, result);
    ctx.arm.cpsr_mut().set_negative(result.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(result == 0);

    1
}
