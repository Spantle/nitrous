use crate::nds::cpu::arm9::{instructions::models::Instruction, Arm9};

mod instructions;
mod lookup;
mod shifter_operand;

pub use lookup::lookup;

struct DataProcessingInstruction {
    is_immediate: bool,
    set_condition_codes: bool,
    first_source_register: u8,
    destination_register: u8,
    second_source_operand: u32,
    carry_out: bool,
}

impl DataProcessingInstruction {
    fn new(arm9: &mut Arm9, inst: Instruction) -> Self {
        let shifter_operand = shifter_operand::parse(arm9, &inst);

        DataProcessingInstruction {
            is_immediate: shifter_operand.is_immediate,
            set_condition_codes: inst.get_bit(20),
            first_source_register: inst.get_bits(16, 19),
            destination_register: inst.get_bits(12, 15),
            second_source_operand: shifter_operand.second_source_operand,
            carry_out: shifter_operand.carry_out,
        }
    }
}
