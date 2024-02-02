mod psr;
mod registers;

pub use psr::*;
pub use registers::*;

#[derive(Debug)]
pub enum PipelineState {
    Fetch,
    Decode,
    Execute,
}
