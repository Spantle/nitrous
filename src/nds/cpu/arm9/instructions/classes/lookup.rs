use crate::nds::{
    cpu::{
        arm9::{instructions::models::Instruction, Arm9},
        bus::Bus,
    },
    logger,
};

use super::{data_processing, load_store};

#[inline(always)]
pub fn lookup_instruction_class<const INST_SET: u16>(
    inst: Instruction,
    arm9: &mut Arm9,
    bus: &mut Bus,
) -> u32 {
    let class = (INST_SET >> 5) & 0b111;

    match class {
        0b000 => {
            // Data Processing (immediate shift / register shift)
            data_processing::lookup::<INST_SET, false>(inst, arm9)
        }
        0b001 => {
            // Data Processing (immediate)
            data_processing::lookup::<INST_SET, true>(inst, arm9)
        }
        0b010 => {
            // Load/Store word or unsigned byte (immediate offset/index)
            load_store::lookup::<INST_SET, false>(inst, arm9, bus)
        }
        0b011 => {
            // Load/Store word or unsigned byte ("register offset/index" / "scaled register offset/index")
            load_store::lookup::<INST_SET, true>(inst, arm9, bus)
        }
        _ => {
            logger::warn(format!("unknown instruction class {:03b}", class));
            1
        }
    }
}
