mod addressing_mode;
mod instructions;
mod lookup;

pub use lookup::lookup;

use crate::nds::cpu::arm9::models::{Context, ContextTrait, DisassemblyTrait, Instruction};

pub struct LoadStoreInstruction {
    pub first_source_register: u8, // Rn
    pub destination_register: u8,  // Rd
    pub addressing_mode: u32,      // bits[11:0] result
}

impl LoadStoreInstruction {
    fn new<const IS_REGISTER: bool>(ctx: &mut Context<Instruction, impl ContextTrait>) -> Self {
        let first_source_register = ctx.inst.get_byte(16, 19);
        ctx.dis.push_reg_end_arg(first_source_register, "[");

        let addressing_mode = if IS_REGISTER {
            addressing_mode::parse_register(ctx)
        } else {
            addressing_mode::parse_immediate(ctx)
        };

        LoadStoreInstruction {
            first_source_register,
            destination_register: ctx.inst.get_byte(12, 15),
            addressing_mode,
        }
    }
}
