use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    logger::LoggerTrait,
};

use super::{instructions, DataProcessingInstruction};

#[inline(always)]
pub fn lookup<const IS_IMMEDIATE: bool, Ctx: ContextTrait>(
    inst_set: u16,
    ctx: &mut Context<Instruction, Ctx>,
) -> u32 {
    let mut ctx = Context::new(
        DataProcessingInstruction::new::<IS_IMMEDIATE>(ctx),
        ctx.arm,
        ctx.bus,
        ctx.shared,
        ctx.dis,
        ctx.logger,
    );

    // cycles are the same for all data-processing instructions
    let cycles = 1 + (!IS_IMMEDIATE) as u32 + ((ctx.inst.destination_register == 15) as u32 * 2);

    let opcode = (inst_set >> 1) & 0b1111;
    let s = inst_set & 1 != 0;

    // TODO: check if this is optimized away
    if s {
        ctx.dis.set_inst_suffix("S");
    }

    match (opcode, s) {
        (0b0000, false) => {
            instructions::and::<false>(&mut ctx);
        }
        (0b0000, true) => {
            instructions::and::<true>(&mut ctx);
        }
        (0b0001, false) => {
            instructions::eor::<false>(&mut ctx);
        }
        (0b0001, true) => {
            instructions::eor::<true>(&mut ctx);
        }
        (0b0010, false) => {
            instructions::sub::<false>(&mut ctx);
        }
        (0b0010, true) => {
            instructions::sub::<true>(&mut ctx);
        }
        (0b0011, true) => {
            instructions::rsb::<true>(&mut ctx);
        }
        (0b0011, false) => {
            instructions::rsb::<false>(&mut ctx);
        }
        (0b0100, false) => {
            instructions::add::<false>(&mut ctx);
        }
        (0b0100, true) => {
            instructions::add::<true>(&mut ctx);
        }
        (0b0101, false) => {
            instructions::adc::<false>(&mut ctx);
        }
        (0b0101, true) => {
            instructions::adc::<true>(&mut ctx);
        }
        (0b0110, false) => {
            instructions::sbc::<false>(&mut ctx);
        }
        (0b0110, true) => {
            instructions::sbc::<true>(&mut ctx);
        }
        (0b0111, false) => {
            instructions::rsc::<false>(&mut ctx);
        }
        (0b0111, true) => {
            instructions::rsc::<true>(&mut ctx);
        }
        (0b1000, false) => {
            instructions::tst(&mut ctx);
        }
        (0b1000, true) => {
            instructions::tst(&mut ctx);
        }
        (0b1001, false) => {
            instructions::teq(&mut ctx);
        }
        (0b1001, true) => {
            instructions::teq(&mut ctx);
        }
        (0b1010, false) => {
            instructions::cmp(&mut ctx);
        }
        (0b1010, true) => {
            instructions::cmp(&mut ctx);
        }
        (0b1011, false) => {
            instructions::cmn(&mut ctx);
        }
        (0b1011, true) => {
            instructions::cmn(&mut ctx);
        }
        (0b1100, false) => {
            instructions::orr::<false>(&mut ctx);
        }
        (0b1100, true) => {
            instructions::orr::<true>(&mut ctx);
        }
        (0b1101, false) => {
            instructions::mov::<false>(&mut ctx);
        }
        (0b1101, true) => {
            instructions::mov::<true>(&mut ctx);
        }
        (0b1110, false) => {
            instructions::bic::<false>(&mut ctx);
        }
        (0b1110, true) => {
            instructions::bic::<true>(&mut ctx);
        }
        (0b1111, false) => {
            instructions::mvn::<false>(&mut ctx);
        }
        (0b1111, true) => {
            instructions::mvn::<true>(&mut ctx);
        }
        _ => {
            ctx.logger
                .log_error(format!("unknown data-processing opcode {:04b}", opcode));
        }
    };

    cycles
}

#[inline(always)]
pub fn lookup_multiply<Ctx: ContextTrait>(
    inst_set: u16,
    ctx: &mut Context<Instruction, Ctx>,
) -> u32 {
    let u = inst_set >> 2 & 1 != 0;
    let a = inst_set >> 1 & 1 != 0;
    let s = inst_set & 1 != 0;
    if s {
        ctx.dis.set_inst_suffix("S");
    }

    // bit 23
    match (inst_set >> 3 & 1 == 1, u, a, s) {
        (false, _, false, false) => instructions::mul::<false>(ctx),
        (false, _, false, true) => instructions::mul::<true>(ctx),
        (false, _, true, false) => instructions::mla::<false>(ctx),
        (false, _, true, true) => instructions::mla::<true>(ctx),
        (true, false, false, false) => instructions::umull::<false>(ctx),
        (true, false, false, true) => instructions::umull::<true>(ctx),
        (true, false, true, false) => instructions::umlal::<false>(ctx),
        (true, false, true, true) => instructions::umlal::<true>(ctx),
        (true, true, false, false) => instructions::smull::<false>(ctx),
        (true, true, false, true) => instructions::smull::<true>(ctx),
        (true, true, true, false) => instructions::smlal::<false>(ctx),
        (true, true, true, true) => instructions::smlal::<true>(ctx),
    }
}
