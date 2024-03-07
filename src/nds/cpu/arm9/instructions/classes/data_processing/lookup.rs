use crate::nds::{
    cpu::arm9::models::{Context, ContextTrait, DisassemblyTrait, Instruction},
    logger::LoggerTrait,
};

use super::{instructions, DataProcessingInstruction};

#[inline(always)]
pub fn lookup<const IS_IMMEDIATE: bool, Ctx: ContextTrait>(
    inst_set: u16,
    ctx: &mut Context<Instruction, Ctx>,
) -> u32 {
    let mut ctx = Context::<_, Ctx> {
        inst: DataProcessingInstruction::new::<IS_IMMEDIATE>(ctx),
        arm9: ctx.arm9,
        bus: ctx.bus,
        dis: ctx.dis,
        logger: ctx.logger,
    };

    // cycles are the same for all data-processing instructions
    let cycles = 1 + (!IS_IMMEDIATE) as u32 + ((ctx.inst.destination_register == 15) as u32 * 2);

    let opcode = (inst_set >> 1) & 0b1111;
    let s = inst_set & 1 != 0;

    // TODO: check if this is optimized away
    if s {
        ctx.dis.set_inst_suffix("S");
    }

    match (opcode, s) {
        (0b0010, false) => {
            instructions::sub::<false>(&mut ctx);
        }
        (0b0010, true) => {
            instructions::sub::<true>(&mut ctx);
        }
        (0b0100, false) => {
            instructions::add::<false>(&mut ctx);
        }
        (0b0100, true) => {
            instructions::add::<true>(&mut ctx);
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
        _ => {
            ctx.logger
                .log_warn(format!("unknown data-processing opcode {:04b}", opcode));
        }
    };

    cycles
}
