mod bits;
mod ctx;
mod disassembly;
mod instruction;
mod keyinput;
mod powcnt;
mod psr;
mod registers;

pub use bits::*;
pub use ctx::*;
pub use disassembly::*;
pub use instruction::*;
pub use keyinput::*;
pub use powcnt::*;
pub use psr::*;
pub use registers::*;

#[derive(Debug)]
pub enum PipelineState {
    Fetch,
    Decode,
    Execute,
}
