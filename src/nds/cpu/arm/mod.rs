#[allow(clippy::module_inception)]
mod arm; // this is intentional shut up
pub mod bus;
mod instructions;
pub mod models;

pub use arm::{Arm, ArmBool, ArmKind, FakeArm};
pub use instructions::lookup_instruction_set;
