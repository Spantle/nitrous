mod powcnt;
mod psr;
mod registers;

pub use powcnt::*;
pub use psr::*;
pub use registers::*;

#[derive(Debug)]
pub enum PipelineState {
    Fetch,
    Decode,
    Execute,
}
