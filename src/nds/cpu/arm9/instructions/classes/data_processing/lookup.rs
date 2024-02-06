use crate::nds::{
    cpu::arm9::{models::Instruction, Arm9},
    logger,
};

use super::{instructions, DataProcessingInstruction};

#[inline(always)]
pub fn lookup<const IS_IMMEDIATE: bool>(inst_set: u16, inst: Instruction, arm9: &mut Arm9) -> u32 {
    let inst = DataProcessingInstruction::new::<IS_IMMEDIATE>(&*arm9, inst);
    // cycles are the same for all data-processing instructions
    let cycles = 1 + (!IS_IMMEDIATE) as u32 + ((inst.destination_register == 15) as u32 * 2);

    let opcode = (inst_set >> 1) & 0b1111;
    let s = inst_set & 1 != 0;
    match (opcode, s) {
        (0b0100, false) => {
            instructions::add::<false>(inst, arm9);
        }
        (0b0100, true) => {
            instructions::add::<true>(inst, arm9);
        }
        (0b1101, false) => {
            instructions::mov::<false>(inst, arm9);
        }
        (0b1101, true) => {
            instructions::mov::<true>(inst, arm9);
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
