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
        0b10 => {
            // ASR (1)
            instructions::asr_1(ctx)
        }
        0b11 => {
            match ((inst_set >> 4) & 0b1, (inst_set >> 3) & 0b1) {
                (0, 0) => {
                    // ADD (3)
                    instructions::add_3(ctx)
                }
                (0, 1) => {
                    // SUB (3)
                    instructions::sub_3(ctx)
                }
                (1, 0) => {
                    // ADD (1)
                    instructions::add_1(ctx)
                }
                (1, 1) => {
                    // SUB (1)
                    instructions::sub_1(ctx)
                }
                _ => unreachable!(),
            }
        }
        _ => {
            ctx.logger.log_error(format!(
                "unknown data processing lookup opcode {:02b}",
                opcode
            ));
            10000
        }
    }
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
        0b10 => {
            // ADD
            instructions::add_2(ctx)
        }
        0b11 => {
            // SUB
            instructions::sub_2(ctx)
        }
        _ => unreachable!(),
    };

    1 // TODO: this is wrong
}

#[inline(always)]
pub fn lookup_register(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let opcode = inst_set & 0b1111;
    match opcode {
        0b0000 => instructions::and(ctx),
        0b0001 => instructions::eor(ctx),
        0b0010 => instructions::lsl_2(ctx),
        0b0011 => instructions::lsr_2(ctx),
        0b0100 => instructions::asr_2(ctx),
        0b0101 => instructions::adc(ctx),
        0b0110 => instructions::sbc(ctx),
        0b0111 => instructions::ror(ctx),
        0b1000 => instructions::tst(ctx),
        0b1001 => instructions::neg(ctx),
        0b1010 => instructions::cmp_2(ctx),
        0b1011 => instructions::cmn(ctx),
        0b1100 => instructions::orr(ctx),
        0b1101 => instructions::mul(ctx),
        0b1110 => instructions::bic(ctx),
        0b1111 => instructions::mvn(ctx),
        _ => {
            ctx.logger.log_error(format!(
                "unknown data processing register lookup opcode {:04b}",
                opcode
            ));
            10000
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
            ctx.logger.log_error(format!(
                "unknown special data-processing opcode {:02b}",
                opcode
            ));
            return 10000;
        }
    };

    1 // TODO: this is wrong
}

#[inline(always)]
pub fn lookup_add_to_sp_or_pc(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let reg = (inst_set >> 5) & 0b1;
    if reg == 1 {
        instructions::add_6(ctx)
    } else {
        instructions::add_5(ctx)
    }
}

#[inline(always)]
pub fn lookup_adjust_stack_pointer(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let opcode = (inst_set >> 1) & 0b1;
    match opcode {
        0 => {
            // ADD (7)
            instructions::add_7(ctx)
        }
        1 => {
            // SUB (4)
            instructions::sub_4(ctx)
        }
        _ => unreachable!(),
    }
}
