use crate::nds::{
    cpu::{
        arm9::{
            arm9::Arm9Trait,
            models::{Context, DisassemblyTrait, Instruction},
        },
        bus::BusTrait,
    },
    logger,
};

use super::{branch, data_processing, load_store};

#[inline(always)]
pub fn lookup_instruction_class(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl Arm9Trait, impl BusTrait, impl DisassemblyTrait>,
) -> u32 {
    let class = (inst_set >> 5) & 0b111;

    match class {
        0b000 => {
            // Data Processing (immediate shift / register shift)
            data_processing::lookup::<false>(inst_set, ctx)
        }
        0b001 => {
            // Data Processing (immediate)
            data_processing::lookup::<true>(inst_set, ctx)
        }
        0b010 => {
            // Load/Store word or unsigned byte (immediate offset/index)
            load_store::lookup::<false>(inst_set, ctx)
        }
        0b011 => {
            // Load/Store word or unsigned byte ("register offset/index" / "scaled register offset/index")
            load_store::lookup::<true>(inst_set, ctx)
        }
        0b101 => {
            // Branch
            branch::lookup(inst_set, ctx)
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
