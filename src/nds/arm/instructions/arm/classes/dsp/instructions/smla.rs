use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// SMLA
pub fn smla(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("SMLA");

    let rm = ctx.inst.get_byte(0, 3);
    let rs = ctx.inst.get_byte(8, 11);
    let rn = ctx.inst.get_byte(12, 15);
    let rd = ctx.inst.get_byte(16, 19);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rm, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rs, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rn, None);

    let rm = ctx.arm.er(rm);
    let rs = ctx.arm.er(rs);

    let x = ctx.inst.get_bit(5);
    let y = ctx.inst.get_bit(6);
    let (operand1, operand2) = match (x, y) {
        (false, false) => {
            ctx.dis.set_inst_suffix("BB");
            (rm.get_bits(0, 15), rs.get_bits(0, 15))
        }
        (false, true) => {
            ctx.dis.set_inst_suffix("BT");
            (rm.get_bits(0, 15), rs.get_bits(16, 31))
        }
        (true, false) => {
            ctx.dis.set_inst_suffix("TB");
            (rm.get_bits(16, 31), rs.get_bits(0, 15))
        }
        (true, true) => {
            ctx.dis.set_inst_suffix("TT");
            (rm.get_bits(16, 31), rs.get_bits(16, 31))
        }
    };
    let (operand1, operand2) = (
        operand1 as u16 as i16 as i32 as u32,
        operand2 as u16 as i16 as i32 as u32,
    );

    let rn = ctx.arm.er(rn);
    let (result, overflow) = (operand1 * operand2).overflowing_add(rn);
    ctx.arm.set_r(rd, result);
    ctx.arm.cpsr_mut().set_overflow(overflow);

    1 // TODO: this is wrong
}