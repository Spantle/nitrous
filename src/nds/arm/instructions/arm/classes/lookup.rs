use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait},
        ArmBool,
    },
    logger::LoggerTrait,
};

use super::{
    branch, coprocessor, data_processing, dsp, exceptions, load_store, load_store_multiple, misc,
    semaphore, status_register_access,
};

#[inline(always)]
pub fn lookup_instruction_class(
    arm_bool: bool,
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let class = (inst_set >> 5) & 0b111;

    match class {
        0b000 => {
            if !ctx.inst.get_bit(4) || !ctx.inst.get_bit(7) {
                // bit 20, and bits 23-24
                if inst_set & 1 == 0 && (inst_set >> 3) & 0b11 == 0b10 {
                    // Miscellaneous
                    return lookup_miscellaneous_instructions(arm_bool, inst_set, ctx);
                } else {
                    // Data Processing (immediate shift / register shift)
                    return data_processing::lookup::<false, _>(inst_set, ctx);
                }
            }

            lookup_multiples_and_extra_load_store_instructions(inst_set, ctx)
        }
        0b001 => {
            // Data Processing (immediate)
            // bit 20-21, and bits 23-24
            if inst_set & 0b11 == 0b10 && (inst_set >> 3) & 0b11 == 0b10 {
                // Miscellaneous
                return status_register_access::instructions::msr(inst_set, ctx);
            }

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
            load_store_multiple::lookup::<_>(arm_bool, inst_set, ctx)
        }
        0b101 => {
            // Branch
            // bits 28-31
            if (inst_set >> 8) & 0b1111 == 0b1111 {
                // Branch with link and change to Thumb (BLX (1))
                branch::instructions::b::<true, true>(ctx)
            } else {
                branch::lookup(inst_set, ctx)
            }
        }
        0b111 => {
            // bit 24
            if (inst_set >> 4) & 1 == 0 {
                // Coprocessor data processing
                coprocessor::lookup(inst_set, ctx)
            } else {
                // Software interrupt
                // technically also possibly an undefined instruction based on the cond or something iirc
                exceptions::instructions::swi(arm_bool, ctx)
            }
        }
        _ => {
            ctx.logger
                .log_error(format!("unknown instruction class {:03b}", class));
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
    // TODO: organize this better, try to reuse bits 6 and 5
    // there's also problems with halfword_or_ibyte's categorization

    if !ctx.inst.get_bit(6) {
        if !ctx.inst.get_bit(5) {
            // bit 24
            if (inst_set >> 4) & 1 == 0 {
                // Multiply
                return data_processing::lookup_multiply(inst_set, ctx);
            } else {
                // Semaphore
                return semaphore::lookup(inst_set, ctx);
            }
        } else {
            // bit 22
            if (inst_set >> 2) & 1 == 0 {
                // Load/store halfword (register offset)
                return load_store::halfword_or_ibyte::lookup::<false, _>(inst_set, ctx);
            } else {
                // Load/store halfword (immediate offset)
                return load_store::halfword_or_ibyte::lookup::<true, _>(inst_set, ctx);
            }
        }
    }

    // else

    // bit 20
    if inst_set & 1 == 0 {
        // Load/store two words
        ctx.logger
            .log_error("load/store two words instruction not implemented");
        // bit 22
        if (inst_set >> 2) & 1 == 0 {
            // Load/store two words (register offset)
            0
        } else {
            // Load/store two words (immediate offset)
            1
        }
    } else {
        // bit 22
        if (inst_set >> 2) & 1 == 0 {
            // Load signed halfword/byte (register offset)
            load_store::halfword_or_ibyte::lookup::<false, _>(inst_set, ctx)
        } else {
            // Load signed halfword/byte (immediate offset)
            load_store::halfword_or_ibyte::lookup::<true, _>(inst_set, ctx)
        }
    }
}

#[inline(always)]
fn lookup_miscellaneous_instructions(
    arm_bool: bool,
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let bits = ctx.inst.get_byte(4, 7);
    match bits {
        0b0000 => {
            // bit 21
            if (inst_set >> 1) & 1 == 0 {
                // Move status register to register
                status_register_access::instructions::mrs(inst_set, ctx)
            } else {
                // Move register to status register
                status_register_access::instructions::msr(inst_set, ctx)
            }
        }
        0b0001 => {
            // bit 22
            if (inst_set >> 2) & 1 == 0 {
                // Branch/exchange instruction set
                branch::instructions::bx::<false>(ctx)
            } else if arm_bool == ArmBool::ARM9 {
                // Count leading zeroes
                misc::instructions::clz(ctx)
            } else {
                ctx.logger
                    .log_error("clz on armv4 undefined instruction not implemented");
                1
            }
        }
        0b0011 => {
            // Branch and link/exchange instruction set (BLX (2))
            branch::instructions::bx::<true>(ctx)
        }
        0b0101 => {
            // Enhanced DSP add/subtracts
            ctx.logger
                .log_error("enhanced DSP add/subtracts instruction not implemented");
            0
        }
        0b0111 => {
            // Software breakpoint
            exceptions::instructions::bkpt(arm_bool, ctx)
        }
        0b1000 | 0b1010 | 0b1100 | 0b1110 => {
            // multiplies are identified by 0b1xy0

            // Enhanced DSP multiplies
            if arm_bool {
                dsp::lookup_multiplies(inst_set, ctx)
            } else {
                ctx.logger
                    .log_error("tried running an enhanced DSP instruction on ARM7");
                10
            }
        }
        _ => {
            ctx.logger
                .log_error(format!("unknown miscellaneous instruction {:04b}", bits));
            1
        }
    }
}
