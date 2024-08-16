use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    logger::LoggerTrait,
};

use super::{
    branch, coprocessor, data_processing, exceptions, load_store, load_store_multiple, misc,
    status_register_access,
};

#[inline(always)]
pub fn lookup_instruction_class(
    _arm_bool: bool,
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let class = (inst_set >> 5) & 0b111;

    match class {
        0b000 => {
            if !ctx.inst.get_bit(4) || !ctx.inst.get_bit(7) {
                // bit 20, and bits 23-24
                if inst_set & 1 == 0 && inst_set >> 3 & 0b11 == 0b10 {
                    // Miscellaneous
                    return lookup_miscellaneous_instructions(inst_set, ctx);
                } else {
                    // Data Processing (immediate shift / register shift)
                    return data_processing::lookup::<false, _>(inst_set, ctx);
                }
            }

            lookup_multiples_and_extra_load_store_instructions(inst_set, ctx)
        }
        0b001 => {
            // Data Processing (immediate)
            if inst_set & 0b11 == 0b10 && inst_set >> 3 & 0b11 == 0b10 {
                // Miscellaneous
                return lookup_miscellaneous_instructions(inst_set, ctx);
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
            load_store_multiple::lookup::<_>(inst_set, ctx)
        }
        0b101 => {
            // Branch
            // bits 28-31
            if inst_set >> 8 & 0b1111 == 0b1111 {
                // Branch with link and change to Thumb (BLX (1))
                branch::instructions::b::<true, true>(ctx)
            } else {
                branch::lookup(inst_set, ctx)
            }
        }
        0b111 => {
            // bit 24
            if inst_set >> 4 & 1 == 0 {
                // Coprocessor data processing
                coprocessor::lookup(inst_set, ctx)
            } else {
                // Software interrupt
                // technically also possibly an undefined instruction based on the cond or something iirc
                exceptions::instructions::swi(ctx)
            }
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
    // TODO: organize this better, try to reuse bits 6 and 5
    // there's also problems with halfword_or_ibyte's categorization

    if !ctx.inst.get_bit(6) {
        if !ctx.inst.get_bit(5) {
            // bit 24
            if inst_set >> 4 & 1 == 0 {
                // Multiply

                let s = inst_set & 1 != 0;
                if s {
                    ctx.dis.set_inst_suffix("S");
                }

                // bit 23
                return match (inst_set >> 3 & 1 == 1, s) {
                    (false, false) => data_processing::instructions::mul::<false>(ctx),
                    (false, true) => data_processing::instructions::mul::<true>(ctx),
                    (true, _) => {
                        // Multiply long
                        ctx.logger
                            .log_warn("multiply long instruction not implemented");
                        return 1;
                    }
                };
            } else {
                // Semaphore
                ctx.logger.log_warn("semaphore instruction not implemented");
                return 1;
            }
        } else {
            // bit 22
            if inst_set >> 2 & 1 == 0 {
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
            .log_warn("load/store two words instruction not implemented");
        // bit 22
        if inst_set >> 2 & 1 == 0 {
            // Load/store two words (register offset)
            0
        } else {
            // Load/store two words (immediate offset)
            1
        }
    } else {
        // bit 22
        if inst_set >> 2 & 1 == 0 {
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
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let bits = ctx.inst.get_byte(4, 7);
    match bits {
        0b0000 => {
            // bit 21
            if inst_set >> 1 & 1 == 0 {
                // Move status register to register
                status_register_access::instructions::mrs(inst_set, ctx)
            } else {
                // Move register to status register
                status_register_access::instructions::msr(inst_set, ctx)
            }
        }
        0b0001 => {
            // bit 22
            if inst_set >> 2 & 1 == 0 {
                // Branch/exchange instruction set
                branch::instructions::bx::<false>(ctx)
            } else {
                // Count leading zeroes
                misc::instructions::clz(ctx)
            }
        }
        0b0011 => {
            // Branch and link/exchange instruction set (BLX (2))
            branch::instructions::bx::<true>(ctx)
        }
        0b0101 => {
            // Enhanced DSP add/subtracts
            ctx.logger
                .log_warn("enhanced DSP add/subtracts instruction not implemented");
            0
        }
        0b0111 => {
            // Software breakpoint
            ctx.logger
                .log_warn("software breakpoint instruction not implemented");
            0
        }
        0b1000 | 0b1010 | 0b1100 | 0b1110 => {
            // Enhanced DSP multiplies
            ctx.logger
                .log_warn("enhanced DSP multiplies instruction not implemented");
            0
        }
        _ => {
            ctx.logger
                .log_warn(format!("unknown miscellaneous instruction {:04b}", bits));
            1
        }
    }
}
