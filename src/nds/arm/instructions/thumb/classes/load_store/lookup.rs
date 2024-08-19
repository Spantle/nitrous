use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait},
    },
    logger::LoggerTrait,
};

use super::instructions;

#[inline(always)]
pub fn lookup_register_offset(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let opcode = (inst_set >> 3) & 0b111;
    match opcode {
        0b001 => instructions::strh_2(ctx),
        0b101 => instructions::ldrh_2(ctx),
        _ => {
            ctx.logger.log_warn(format!(
                "unknown load/store register offset opcode {:03b}",
                opcode
            ));
            10000
        }
    }
}

#[inline(always)]
pub fn lookup_word_byte_immediate(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let bl = (inst_set >> 5) & 0b11;
    match bl {
        0b00 => {
            instructions::str_1(ctx);
        }
        0b01 => {
            instructions::ldr_1(ctx);
        }
        0b10 => {
            instructions::strb_1(ctx);
        }
        0b11 => {
            instructions::ldrb_1(ctx);
        }
        _ => unreachable!(),
    }

    1 // TODO: this is wrong
}

#[inline(always)]
pub fn lookup_halfword_immediate(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let l = (inst_set >> 5) & 0b1 == 1;
    if l {
        instructions::ldrh_1(ctx);
    } else {
        instructions::strh_1(ctx);
    }

    1 // TODO: this is wrong
}
