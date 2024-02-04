use crate::nds::{
    cpu::{
        arm9::{instructions::models::Instruction, Arm9},
        bus::Bus,
    },
    logger,
};

use super::{branch, data_processing, load_store};

#[inline(always)]
pub fn lookup_instruction_class(
    inst_set: u16,
    inst: Instruction,
    arm9: &mut Arm9,
    bus: &mut Bus,
) -> u32 {
    let class = (inst_set >> 5) & 0b111;

    match class {
        0b000 => {
            // Data Processing (immediate shift / register shift)
            data_processing::lookup::<false>(inst_set, inst, arm9)
        }
        0b001 => {
            // Data Processing (immediate)
            data_processing::lookup::<true>(inst_set, inst, arm9)
        }
        0b010 => {
            // Load/Store word or unsigned byte (immediate offset/index)
            load_store::lookup::<false>(inst_set, inst, arm9, bus)
        }
        0b011 => {
            // Load/Store word or unsigned byte ("register offset/index" / "scaled register offset/index")
            load_store::lookup::<true>(inst_set, inst, arm9, bus)
        }
        0b101 => {
            // Branch
            branch::lookup(inst_set, inst, arm9)
        }
        _ => {
            logger::warn(
                logger::LogSource::Arm9,
                format!("unknown instruction class {:03b}", class),
            );
            1
        }
    }
}
