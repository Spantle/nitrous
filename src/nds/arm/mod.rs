#[allow(clippy::module_inception)]
mod arm; // this is intentional shut up
mod fake;
pub mod instructions;
pub mod models;
mod rw;
mod t;

pub use arm::Arm;
pub use fake::FakeArm;
pub use models::{ArmBool, ArmKind};
pub use rw::ArmInternalRW;
pub use t::ArmTrait;
