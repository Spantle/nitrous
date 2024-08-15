use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// ROR
pub fn ror(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("ROR");

    let rd = ctx.inst.get_byte(0, 2);
    let rs = ctx.inst.get_byte(3, 5);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rs, None);

    let rs = ctx.arm.r()[rs];
    let result = if rs.get_bits(0, 7) == 0 {
        ctx.arm.r()[rd]
    } else {
        let bits = rs.get_bits(0, 4);
        if bits == 0 {
            let rd = ctx.arm.r()[rd];
            ctx.arm.cpsr_mut().set_carry(rd.get_bit(31));
            rd
        } else {
            let rd = ctx.arm.r()[rd];
            ctx.arm.cpsr_mut().set_carry(rd.get_bit(bits - 1));
            rd.rotate_right(bits)
        }
    };

    ctx.arm.set_r(rd, result);
    ctx.arm.cpsr_mut().set_negative(result.get_bit(31));
    ctx.arm.cpsr_mut().set_zero(result == 0);

    1
}
