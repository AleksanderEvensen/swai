use swai_parser::instructions::Instructions;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WasmInterpreterError {
    #[error("Failed to parse bytes: '{bytes:?}' into a string")]
    StringFromBytes { bytes: Vec<u8> },

    #[error("Tried to set memory data ({data:?}) at offset ({offset}) failed to set byte at index: {failed_pos} of total memory length ({memory_len})")]
    ModifyMemoryOutOfBounds {
        offset: usize,
        data: Vec<u8>,
        failed_pos: usize,
        memory_len: usize,
    },

    #[error("Memory invalid with unknown offset expr: {0:#?}")]
    InvalidMemorySegmentOffset(Instructions),

    #[error("The module doesn't have an entry point 'start' function")]
    NoEntryPoint,

    #[error("I/O error: {0:#?}")]
    IOError(#[from] std::io::Error),
}
