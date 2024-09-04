#[allow(clippy::module_inception)]
mod arm; // this is intentional shut up
pub mod instructions;
pub mod models;

pub use arm::{Arm, ArmBool, ArmKind, FakeArm};
