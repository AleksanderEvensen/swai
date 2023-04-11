use bytereader::ByteReaderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WasmParserError {
    #[error("Failed to parse bytes: '{bytes:?}' into a string")]
    StringFromBytes { bytes: Vec<u8> },

    #[error("A section wasn't structured or parsed the correct way: '{message}'")]
    InvalidSectionError { message: &'static str },

    #[error("Unknown section id: {id}")]
    InvalidSectionId { id: u8 },

    #[error("Invalid wasm bytes: '{message}'")]
    InvalidWasmBytes { message: &'static str },

    #[error("Failed to parse wasm bytes. Reader error: {0}")]
    ParserError(#[from] ByteReaderError),
}
