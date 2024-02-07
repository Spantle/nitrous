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

use super::{instructions, DataProcessingInstruction};

#[inline(always)]
pub fn lookup<const IS_IMMEDIATE: bool>(
    inst_set: u16,
    ctx: &mut Context<Instruction, impl Arm9Trait, impl BusTrait, impl DisassemblyTrait>,
) -> u32 {
    let mut ctx = Context {
        inst: DataProcessingInstruction::new::<IS_IMMEDIATE>(ctx),
        arm9: ctx.arm9,
        bus: ctx.bus,

        dis: ctx.dis,
    };

    // cycles are the same for all data-processing instructions
    let cycles = 1 + (!IS_IMMEDIATE) as u32 + ((ctx.inst.destination_register == 15) as u32 * 2);

    let opcode = (inst_set >> 1) & 0b1111;
    let s = inst_set & 1 != 0;
    match (opcode, s) {
        (0b0100, false) => {
            ctx.dis.set_inst("ADD");
            instructions::add::<false>(&mut ctx);
        }
        (0b0100, true) => {
            ctx.dis.set_inst("ADDS");
            instructions::add::<true>(&mut ctx);
        }
        (0b1101, false) => {
            ctx.dis.set_inst("MOV");
            instructions::mov::<false>(&mut ctx);
        }
        (0b1101, true) => {
            ctx.dis.set_inst("MOVS");
            instructions::mov::<true>(&mut ctx);
        }
        _ => {
            logger::warn(
                logger::LogSource::Arm9,
                format!("unknown data-processing opcode {:04b}", opcode),
            );
        }
    };

    cycles
}
