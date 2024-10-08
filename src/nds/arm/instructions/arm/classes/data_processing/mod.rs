use crate::nds::arm::{
    instructions::arm::Instruction,
    models::{Context, ContextTrait},
};

mod instructions;
mod lookup;
mod shifter_operand;

pub use lookup::*;

pub struct DataProcessingInstruction {
    first_source_register: u8,  // Rn
    destination_register: u8,   // Rd
    second_source_operand: u32, // bits[11:0] result
    carry_out: bool,            // C flag
}

impl DataProcessingInstruction {
    fn new<const IS_IMMEDIATE: bool>(ctx: &mut Context<Instruction, impl ContextTrait>) -> Self {
        let shifter_operand = if IS_IMMEDIATE {
            shifter_operand::parse_immediate(ctx)
        } else {
            shifter_operand::parse_register(ctx)
        };

        DataProcessingInstruction {
            first_source_register: ctx.inst.get_byte(16, 19),
            destination_register: ctx.inst.get_byte(12, 15),
            second_source_operand: shifter_operand.second_source_operand,
            carry_out: shifter_operand.carry_out,
        }
    }
}
