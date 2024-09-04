use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// LSR (2)
pub fn lsr_2(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("LSR");

    let rd = ctx.inst.get_byte(0, 2);
    let rs = ctx.inst.get_byte(3, 5);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rs, None);

    let rs = ctx.arm.r()[rs];
    let byte = rs.get_bits(0, 7);
    let result = if byte == 0 {
        ctx.arm.r()[rd]
    } else if byte < 32 {
        let rd = ctx.arm.r()[rd];
        ctx.arm.cpsr_mut().set_carry(rd.get_bit(byte - 1));
        rd >> byte
    } else if byte == 32 {
        let rd = ctx.arm.r()[rd];
        ctx.arm.cpsr_mut().set_carry(rd.get_bit(31));
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
