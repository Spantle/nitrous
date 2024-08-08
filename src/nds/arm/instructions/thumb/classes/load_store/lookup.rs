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
        0b101 => instructions::ldrh_2(ctx),
        _ => {
            ctx.logger.log_warn(format!(
                "unknown load/store register offset opcode {:03b}",
                opcode
            ));
            return 10000;
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
        0b01 => {
            instructions::ldr_1(ctx);
        }
        0b11 => {
            instructions::ldrb_1(ctx);
        }
        _ => {
            ctx.logger.log_warn(format!(
                "unknown load/store word/byte immediate BL {:02b}",
                bl
            ));
            return 10000;
        }
    }

    1 // TODO: this is wrong
}
