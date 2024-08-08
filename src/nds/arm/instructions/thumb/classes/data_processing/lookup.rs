use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait},
    },
    logger::LoggerTrait,
};

use super::instructions;

#[inline(always)]
pub fn lookup(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let opcode = (inst_set >> 5) & 0b11;
    match opcode {
        0b00 => {
            // LSL (1)
            instructions::lsl_1(ctx)
        }
        0b01 => {
            // LSR (1)
            instructions::lsr_1(ctx)
        }
        0b11 => {
            if (inst_set << 4) & 0b1 == 0 {
                // ADD (3)
                instructions::add_3(ctx)
            } else {
                // ADD (1)
                instructions::add_1(ctx)
            }
        }
        _ => {
            ctx.logger.log_warn(format!(
                "unknown data processing lookup opcode {:02b}",
                opcode
            ));
            return 10000;
        }
    };

    1 // TODO: this is wrong
}

#[inline(always)]
pub fn lookup_ascm_immediate(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let opcode = (inst_set >> 5) & 0b11;
    match opcode {
        0b00 => {
            // MOV
            instructions::mov_1(ctx)
        }
        0b01 => {
            // CMP
            instructions::cmp_1(ctx)
        }
        _ => {
            ctx.logger.log_warn(format!(
                "unknown ascm immediate lookup opcode {:02b}",
                opcode
            ));
        }
    };

    1 // TODO: this is wrong
}

#[inline(always)]
pub fn lookup_register(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let opcode = inst_set & 0b1111;
    match opcode {
        0b0000 => instructions::and(ctx),
        0b1100 => instructions::orr(ctx),
        _ => {
            ctx.logger.log_warn(format!(
                "unknown data processing register lookup opcode {:04b}",
                opcode
            ));
            return 10000;
        }
    }
}

#[inline(always)]
pub fn lookup_special(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let opcode = (inst_set >> 2) & 0b11;
    match opcode {
        0b00 => {
            // ADD (4)
            instructions::add_4(ctx)
        }
        0b01 => {
            // CMP (3)
            instructions::cmp_3(ctx)
        }
        0b10 => {
            // MOV (3)
            instructions::mov_3(ctx)
        }
        _ => {
            ctx.logger.log_warn(format!(
                "unknown special data-processing opcode {:02b}",
                opcode
            ));
            return 10000;
        }
    };

    1 // TODO: this is wrong
}
