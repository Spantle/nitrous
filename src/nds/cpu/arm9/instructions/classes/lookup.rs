use crate::nds::{
    cpu::arm9::models::{Context, ContextTrait, Instruction},
    logger::LoggerTrait,
};

use super::{branch, data_processing, load_store};

#[inline(always)]
pub fn lookup_instruction_class(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let class = (inst_set >> 5) & 0b111;

    match class {
        0b000 => {
            // Data Processing (immediate shift / register shift)
            data_processing::lookup::<false, _>(inst_set, ctx)
        }
        0b001 => {
            // Data Processing (immediate)
            data_processing::lookup::<true, _>(inst_set, ctx)
        }
        0b010 => {
            // Load/Store word or unsigned byte (immediate offset/index)
            load_store::lookup::<false, _>(inst_set, ctx)
        }
        0b011 => {
            // Load/Store word or unsigned byte ("register offset/index" / "scaled register offset/index")
            load_store::lookup::<true, _>(inst_set, ctx)
        }
        0b101 => {
            // Branch
            branch::lookup(inst_set, ctx)
        }
        _ => {
            ctx.logger
                .log_warn(format!("unknown instruction class {:03b}", class));
            1
        }
    }
}
