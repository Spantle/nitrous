use crate::nds::{
    cpu::arm9::models::{Context, Instruction},
    logger,
};

use super::{instructions, DataProcessingInstruction};

#[inline(always)]
pub fn lookup<const IS_IMMEDIATE: bool>(inst_set: u16, ctx: Context<Instruction>) -> u32 {
    let ctx = Context {
        inst: DataProcessingInstruction::new::<IS_IMMEDIATE>(&ctx),
        arm9: ctx.arm9,
        bus: ctx.bus,
    };
    // cycles are the same for all data-processing instructions
    let cycles = 1 + (!IS_IMMEDIATE) as u32 + ((ctx.inst.destination_register == 15) as u32 * 2);

    let opcode = (inst_set >> 1) & 0b1111;
    let s = inst_set & 1 != 0;
    match (opcode, s) {
        (0b0100, false) => {
            instructions::add::<false>(ctx);
        }
        (0b0100, true) => {
            instructions::add::<true>(ctx);
        }
        (0b1101, false) => {
            instructions::mov::<false>(ctx);
        }
        (0b1101, true) => {
            instructions::mov::<true>(ctx);
        }
        _ => {
            logger::warn(
                logger::LogSource::Arm9,
                format!("unknown data-processing opcode {:04b}", opcode),
            );
        }
    };

    cycles
}
