use crate::nds::{
    cpu::arm9::models::{Context, ContextTrait, Instruction},
    logger::LoggerTrait,
};

use super::{branch, data_processing, load_store, load_store_multiple};

#[inline(always)]
pub fn lookup_instruction_class(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let class = (inst_set >> 5) & 0b111;

    match class {
        0b000 => {
            if !ctx.inst.get_bit(4) || !ctx.inst.get_bit(7) {
                // bit 20, and bits 23-24
                if inst_set & 0b1 == 0 && inst_set >> 3 & 0b11 == 0b10 {
                    // Miscellaneous
                } else {
                    // Data Processing (immediate shift / register shift)
                    return data_processing::lookup::<false, _>(inst_set, ctx);
                }
            } else {
                // Multiplies, extra load/stores
                return lookup_multiples_and_extra_load_store_instructions(inst_set, ctx);
            }

            // Data Processing (immediate shift / register shift)
            data_processing::lookup::<false, _>(inst_set, ctx)
        }
        0b001 => {
            // Data Processing (immediate)
            data_processing::lookup::<true, _>(inst_set, ctx)
        }
        0b010 => {
            // Load/Store word or unsigned byte (immediate offset/index)
            load_store::word_or_ubyte::lookup::<false, _>(inst_set, ctx)
        }
        0b011 => {
            // Load/Store word or unsigned byte ("register offset/index" / "scaled register offset/index")
            load_store::word_or_ubyte::lookup::<true, _>(inst_set, ctx)
        }
        0b100 => {
            // Load/Store Multiple
            load_store_multiple::lookup::<_>(inst_set, ctx)
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

#[inline(always)]
fn lookup_multiples_and_extra_load_store_instructions(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    // agony

    if !ctx.inst.get_bit(6) {
        if !ctx.inst.get_bit(5) {
            // bit 24
            if inst_set >> 4 & 0b1 == 0 {
                // Multiply
                0
            } else {
                // Semaphore
                1
            }
        } else {
            // bit 22
            if inst_set >> 2 & 0b1 == 0 {
                // Load/store halfword (register offset)
                load_store::halfword_or_ibyte::lookup::<false, _>(inst_set, ctx)
            } else {
                // Load/store halfword (immediate offset)
                load_store::halfword_or_ibyte::lookup::<true, _>(inst_set, ctx)
            }
        }
    } else {
        // bit 20
        if inst_set & 0b1 == 0 {
            // Load/store two words
            // bit 22
            if inst_set >> 2 & 0b1 == 0 {
                // Load/store two words (register offset)
                0
            } else {
                // Load/store two words (immediate offset)
                1
            }
        } else {
            // bit 22
            if inst_set >> 2 & 0b1 == 0 {
                // Load signed halfword/byte (register offset)
                load_store::halfword_or_ibyte::lookup::<false, _>(inst_set, ctx)
            } else {
                // Load signed halfword/byte (immediate offset)
                load_store::halfword_or_ibyte::lookup::<true, _>(inst_set, ctx)
            }
        }
    }
}
