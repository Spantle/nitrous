use crate::nds::{cpu::arm9::Arm9, logger};

use super::{classes::lookup_instruction_class, conditions::calculate_cond, models::Instruction};

pub fn run_instruction_set<const INST_SET: u16>(arm9: &mut Arm9, inst: Instruction) -> u32 {
    let cond_result = calculate_cond::<INST_SET>(arm9);
    if !cond_result {
        logger::debug("condition failed");
        return 0;
    }

    // also runs it ignore the name
    lookup_instruction_class::<INST_SET>(arm9, inst)
}
