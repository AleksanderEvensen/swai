use bytereader::ByteReaderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WasmParserError {
    #[error("Failed to parse bytes: '{bytes:?}' into a string")]
    StringFromBytes { bytes: Vec<u8> },

    #[error("A section wasn't structured or parsed the correct way: '{message}'")]
    InvalidSectionError { message: String },

    #[error("Unknown section id: {id}")]
    InvalidSectionId { id: u8 },

    #[error("Invalid wasm bytes: '{message}'")]
    InvalidWasmBytes { message: String },

    // From other error types
    #[error("Failed to parse wasm bytes. Reader error: {0:#?}")]
    ParserError(#[from] ByteReaderError),

    #[error("I/O error: {0:#?}")]
    IOError(#[from] std::io::Error),
}
