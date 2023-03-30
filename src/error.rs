use thiserror::Error;

#[derive(Error, Debug)]
pub enum WasmParserError {
    #[error("A parser error occurred")]
    NomParserError(nom::error::ErrorKind),

    #[error("Failed to parse bytes: '{bytes:?}' into a string")]
    StringFromBytes { bytes: Vec<u8> },
}
