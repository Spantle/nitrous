use crate::nds::cpu::arm9::{instructions::models::Instruction, Arm9};

use super::data_processing;

#[inline(always)]
pub fn lookup_instruction_class<const INST_SET: u16>(arm9: &mut Arm9, inst: Instruction) -> u32 {
    let class = (INST_SET >> 5) & 0b111;

    match class {
        0b000 => {
            // Data Processing (immediate shift / register shift)
            data_processing::lookup::<INST_SET, true>(arm9, inst)
        }
        0b001 => {
            // Data Processing (immediate)
            data_processing::lookup::<INST_SET, false>(arm9, inst)
        }
        _ => {
            println!("unknown instruction class {:03b}", class);
            1
        }
    }
}
