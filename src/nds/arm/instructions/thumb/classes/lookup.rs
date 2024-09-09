use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait},
    },
    logger::LoggerTrait,
};

use super::{branch, data_processing, exceptions, load_store, load_store_multiple};

#[inline(always)]
pub fn lookup_instruction_class(
    arm_bool: bool,
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let class = (inst_set >> 7) & 0b111;

    match class {
        0b000 => {
            // Shift by immediate, add/subtract register, add/subtract immediate
            data_processing::lookup(inst_set, ctx)
        }
        0b001 => {
            // Add/subtract/compare/move immediate
            data_processing::lookup_ascm_immediate(inst_set, ctx)
        }
        0b010 => {
            if (inst_set >> 6) & 0b1 == 1 {
                // Load/store register offset
                return load_store::lookup_register_offset(inst_set, ctx);
            }

            if (inst_set >> 5) & 0b1 == 1 {
                // Load from literal pool
                return load_store::instructions::ldr_3(ctx);
            }

            if (inst_set >> 4) & 0b1 == 0 {
                // Data-processing register
                return data_processing::lookup_register(inst_set, ctx);
            }

            if (inst_set >> 2) & 0b11 == 0b11 {
                // Branch/exchange instruction set
                let l = ((inst_set >> 1) & 0b1) == 1;
                if l {
                    return branch::instructions::bx::<true>(ctx);
                } else {
                    return branch::instructions::bx::<false>(ctx);
                }
            }

            data_processing::lookup_special(inst_set, ctx)
        }
        0b011 => {
            // Load/store word/byte immediate offset
            load_store::lookup_word_byte_immediate(inst_set, ctx)
        }
        0b100 => {
            if (inst_set >> 6) & 0b1 == 0 {
                // Load/store halfword immediate offset
                load_store::lookup_halfword_immediate(inst_set, ctx)
            } else {
                // Load/store to/from stack
                load_store::lookup_stack(inst_set, ctx)
            }
        }
        0b101 => {
            if (inst_set >> 6) & 0b1 == 0 {
                // Add to SP or PC
                data_processing::instructions::add_5(ctx);
            }

            if (inst_set >> 4) & 0b1 == 0 {
                // Adjust stack pointer
                ctx.logger.log_error("Adjust stack pointer not implemented");
                return 10000;
            }

            if (inst_set >> 3) & 0b1 == 0 {
                // Push/pop registers
                return load_store_multiple::lookup_push_pop(arm_bool, inst_set, ctx);
            }

            // Software breakpoint
            ctx.logger.log_error("Software breakpoint not implemented");
            10000
        }
        0b110 => {
            if (inst_set >> 6) & 0b1 == 0 {
                // Load/store multiple
                return load_store_multiple::lookup(inst_set, ctx);
            }

            if (inst_set >> 2) & 0b1111 == 0b1111 {
                // Software interrupt
                return exceptions::instructions::swi(arm_bool, ctx);
            }

            branch::instructions::b_1(inst_set, ctx)
        }
        0b111 => {
            // branching shenanigans
            branch::lookup(arm_bool, inst_set, ctx)
        }
        _ => {
            ctx.logger
                .log_error(format!("unknown instruction class {:03b}", class));
            10000
        }
    }
}
