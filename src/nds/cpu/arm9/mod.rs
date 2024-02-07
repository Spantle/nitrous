mod arm9; // this is intentional shut up
mod instructions;
pub mod models;

pub use arm9::{Arm9, FakeArm9};
pub use instructions::lookup_instruction_set;
