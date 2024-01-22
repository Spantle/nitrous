mod addressing_mode;
mod instructions;
mod lookup;

pub use lookup::lookup;

use crate::nds::cpu::arm9::{instructions::models::Instruction, Arm9};

pub struct LoadStoreInstruction {
    pub post_indexing: bool, // P: technically 0 but we've flipped it since 1 is "offset"/"pre-indexed" addressing
    pub is_add: bool,        // U
    pub is_unsigned_byte: bool, // B
    pub w: bool,             // W
    pub is_load: bool,       // L

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
            post_indexing: inst.get_bit(24),
            is_add: inst.get_bit(23),
            is_unsigned_byte: inst.get_bit(22),
            w: inst.get_bit(21),
            is_load: inst.get_bit(20),

            first_source_register: inst.get_bits(16, 19),
            destination_register: inst.get_bits(12, 15),
            addressing_mode,
        }
    }
}
