mod addressing_mode;
mod instructions;
mod lookup;

pub use lookup::lookup;

use crate::nds::cpu::arm9::{instructions::models::Instruction, Arm9};

pub struct LoadStoreInstruction {
    pub first_source_register: u8, // Rn
    pub destination_register: u8,  // Rd
    pub addressing_mode: u32,      // bits[11:0] result
}

impl LoadStoreInstruction {
    fn new<const IS_REGISTER: bool>(arm9: &Arm9, inst: Instruction) -> Self {
        let addressing_mode = if IS_REGISTER {
            addressing_mode::parse_register(arm9, &inst)
        } else {
            addressing_mode::parse_immediate(arm9, &inst)
        };

        LoadStoreInstruction {
            first_source_register: inst.get_byte(16, 19),
            destination_register: inst.get_byte(12, 15),
            addressing_mode,
        }
    }
}