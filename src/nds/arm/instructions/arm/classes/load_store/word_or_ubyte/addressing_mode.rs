use crate::nds::arm::{
    instructions::arm::Instruction,
    models::{Bits, Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

#[inline(always)]
pub fn parse_immediate(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let result = ctx.inst.get_word(0, 11);
    ctx.dis.push_word_end_arg(result, Some(", "));

    result
}

#[inline(always)]
pub fn parse_register(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let rm = ctx.inst.get_byte(0, 3);
    ctx.dis.push_reg_end_arg(rm, Some(", "));
    let rm = ctx.arm.eru(rm);

    let shift = ctx.inst.get_byte(5, 6);
    let shift_imm = ctx.inst.get_word(7, 11);
    let result = match shift {
        0b00 => {
            // LSL
            ctx.dis.push_str_end_arg("LSL", Some(", "));
            rm << shift_imm
        }
        0b01 => {
            // LSR
            ctx.dis.push_str_end_arg("LSR", Some(", "));
            if shift_imm == 0 {
                // LSR #32
                0
            } else {
                rm >> shift_imm
            }
        }
        0b10 => {
            // ASR
            ctx.dis.push_str_end_arg("ASR", Some(", "));
            if shift_imm == 0 {
                // ASR #32
                if rm.get_bit(31) {
                    // technically bit 31 is supposed to equal 1 but we cheat this
                    // update from a few months later: what the fuck are you talking about
                    0xFFFFFFFF
                } else {
                    0
                }
            } else {
                ((rm as i32) >> shift_imm) as u32
            }
        }
        0b11 => {
            if shift_imm == 0 {
                // RRX
                ctx.dis.push_str_end_arg("RRX", Some(", "));
                (ctx.arm.cpsr().get_carry() as u32) << 31 | rm >> 1
            } else {
                // ROR
                ctx.dis.push_str_end_arg("ROR", Some(", "));
                rm.rotate_right(shift_imm)
            }
        }
        _ => unreachable!(),
    };

    ctx.dis.push_word_end_arg(shift_imm, Some(" "));
    result
}
