mod addressing_mode;
mod instructions;
mod lookup;

pub use lookup::lookup;

use crate::nds::arm::models::{Context, ContextTrait, DisassemblyTrait, Instruction};

pub struct LoadStoreInstruction {
    pub is_signed: bool,   // S bit (signed or unsigned)
    pub is_halfword: bool, // H bit (halfword or byte)

    pub first_source_register: u8, // Rn
    pub destination_register: u8,  // Rd
    pub addressing_mode: u32,      // bits[11:0] result
}

impl LoadStoreInstruction {
    fn new<const IS_IMMEDIATE: bool>(
        inst_set: u16,
        ctx: &mut Context<Instruction, impl ContextTrait>,
    ) -> Self {
        let first_source_register = ctx.inst.get_byte(16, 19);
        ctx.dis.push_reg_end_arg(first_source_register, Some("["));

        let post_indexing = inst_set >> 4 & 1 == 0; // P
        if post_indexing {
            ctx.dis.push_str_end_arg("", Some("]"));
        }

        let addressing_mode = if IS_IMMEDIATE {
            addressing_mode::parse_immediate(ctx)
        } else {
            addressing_mode::parse_register(ctx)
        };

        if !post_indexing {
            ctx.dis.push_str_end_arg("", Some("]"));
        }

        LoadStoreInstruction {
            is_signed: ctx.inst.get_bit(6),
            is_halfword: ctx.inst.get_bit(5),

            first_source_register,
            destination_register: ctx.inst.get_byte(12, 15),
            addressing_mode,
        }
    }
}
