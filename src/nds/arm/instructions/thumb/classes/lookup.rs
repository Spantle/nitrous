use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait},
        ArmKind,
    },
    logger::LoggerTrait,
};

use super::{branch, data_processing, load_store};

#[inline(always)]
pub fn lookup_instruction_class(
    arm_kind: ArmKind,
    inst_set: u16,
    ctx: &mut Context<Instruction, impl ContextTrait>,
) -> u32 {
    let class = (inst_set >> 7) & 0b111;

    match class {
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

            ctx.logger
                .log_warn(format!("Unknown 0b010 instruction {:#018b}", inst_set));
            10000
        }
        0b111 => {
            // branching shenanigans
            branch::lookup(arm_kind, inst_set, ctx)
        }
        _ => {
            ctx.logger
                .log_warn(format!("unknown instruction class {:03b}", class));
            10000
        }
    }
}
