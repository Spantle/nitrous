use crate::nds::cpu::arm9::{instructions::models::Instruction, Arm9};

use super::{instructions::mov, DataProcessingInstruction};

#[inline(always)]
pub fn lookup<const INST_SET: u16>(arm9: &mut Arm9, inst: Instruction) -> u32 {
    let inst = DataProcessingInstruction::new(arm9, inst);
    let opcode = (INST_SET >> 1) & 0b1111;
    match opcode {
        0b1101 => mov(arm9, inst),
        _ => {
            println!("unknown opcode");
            1
        }
    }
}
