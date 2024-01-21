use crate::nds::cpu::arm9::{instructions::models::Instruction, Arm9};

mod instructions;
mod lookup;
mod shifter_operand;

pub use lookup::lookup;

pub struct DataProcessingInstruction {
    first_source_register: u8,  // Rn
    destination_register: u8,   // Rd
    second_source_operand: u32, // bits[11:0] result
    carry_out: bool,            // C flag
}

impl DataProcessingInstruction {
    fn new<const IS_IMMEDIATE: bool>(arm9: &mut Arm9, inst: Instruction) -> Self {
        let shifter_operand = if IS_IMMEDIATE {
            shifter_operand::parse_immediate(arm9, &inst)
        } else {
            shifter_operand::parse_register(arm9, &inst)
        };

        DataProcessingInstruction {
            first_source_register: inst.get_bits(16, 19),
            destination_register: inst.get_bits(12, 15),
            second_source_operand: shifter_operand.second_source_operand,
            carry_out: shifter_operand.carry_out,
        }
    }
}
