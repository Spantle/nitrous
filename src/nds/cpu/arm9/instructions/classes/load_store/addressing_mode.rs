use crate::nds::{
    cpu::arm9::{
        arm9::Arm9Trait,
        models::{Context, ContextTrait, DisassemblyTrait, Instruction},
    },
    logger::LoggerTrait,
};

pub fn parse_immediate(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let result = ctx.inst.get_word(0, 11);
    ctx.dis.push_word_end_arg(result, ", ");
    ctx.dis.push_str_end_arg("]", "");

    result
}

pub fn parse_register(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let rm = ctx.inst.get_byte(0, 3);
    ctx.dis.push_reg_end_arg(rm, ", ");
    let rm = ctx.arm9.eru(rm);

    let shift = ctx.inst.get_byte(5, 6);
    let shift_imm = ctx.inst.get_word(7, 11);
    let result = match shift {
        0b00 => {
            // LSL
            ctx.dis.push_str_end_arg("LSL", ", ");
            rm << shift_imm
        }
        0b01 => {
            // LSR
            ctx.dis.push_str_end_arg("LSR", ", ");
            if shift_imm == 0 {
                // LSR #32
                0
            } else {
                rm >> shift_imm
            }
        }
        0b10 => {
            // ASR
            ctx.dis.push_str_end_arg("ASR", ", ");
            if shift_imm == 0 {
                // ASR #32
                if rm & (1 << 31) != 0 {
                    // technically bit 31 is supposed to equal 1 but we cheat this
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
                ctx.dis.push_str_end_arg("RRX", ", ");
                ctx.logger.log_debug("the funny actually happened"); // TODO: remove
                (ctx.arm9.cpsr().get_carry() as u32) << 31 | rm >> 1
            } else {
                // ROR
                ctx.dis.push_str_end_arg("ROR", ", ");
                rm.rotate_right(shift_imm)
            }
        }
        _ => unreachable!(),
    };

    ctx.dis.push_word_end_arg(shift_imm, " ");
    ctx.dis.push_str_end_arg("]", "");
    result
}
