use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait},
    },
    logger::LoggerTrait,
};

use super::{branch, data_processing, load_store, load_store_multiple};

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
            data_processing::ascm_immediate_lookup(inst_set, ctx)
        }
        0b010 => {
            if (inst_set >> 6) & 0b1 == 1 {
                // Load/store register offset
                ctx.logger
                    .log_warn("Load/store register offset not implemented");
                return 1;
            }

            if (inst_set >> 5) & 0b1 == 1 {
                // Load from literal pool
                return load_store::instructions::ldr_3(ctx);
            }

            if (inst_set >> 2) & 0b111 == 0b111 {
                // Branch/exchange instruction set
                let l = ((inst_set >> 1) & 0b1) == 1;
                if l {
                    return branch::instructions::bx::<true>(ctx);
                } else {
                    return branch::instructions::bx::<false>(ctx);
                }
            }

            ctx.logger
                .log_warn(format!("Unknown 0b010 instruction {:#018b}", inst_set));
            10000
        }
        0b100 => {
            if (inst_set >> 6) & 0b1 == 0 {
                // Load/store halfword immediate offset
                load_store::instructions::ldrh_1(ctx)
            } else {
                // Load/store to/from stack
                ctx.logger
                    .log_warn("Load/store to/from stack not implemented");
                return 10000;
            }
        }
        0b101 => {
            if (inst_set >> 6) & 0b1 == 0 {
                // Add to SP or PC
                ctx.logger.log_warn("Add to SP or PC not implemented");
                return 10000;
            }

            if (inst_set >> 4) & 0b1 == 0 {
                // Adjust stack pointer
                ctx.logger.log_warn("Adjust stack pointer not implemented");
                return 10000;
            }

            if (inst_set >> 3) & 0b1 == 0 {
                // Push/pop registers
                return load_store_multiple::lookup_push_pop(arm_bool, inst_set, ctx);
            }

            // Software breakpoint
            ctx.logger.log_warn("Software breakpoint not implemented");
            10000
        }
        0b110 => {
            if (inst_set >> 6) & 0b1 == 0 {
                // Load/store multiple
                ctx.logger.log_warn("Load/store multiple not implemented");
                return 10000;
            }

            if (inst_set >> 2) & 0b1111 == 0b1111 {
                // Software interrupt
                ctx.logger.log_warn("Software interrupt not implemented");
                return 10000;
            }

            branch::instructions::b::<true>(inst_set, ctx)
        }
        0b111 => {
            // branching shenanigans
            branch::lookup(arm_bool, inst_set, ctx)
        }
        _ => {
            ctx.logger
                .log_warn(format!("unknown instruction class {:03b}", class));
            10000
        }
    }
}
