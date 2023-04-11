pub mod sections;
pub mod types;

use crate::error::WasmParserError;
use bytereader::ByteReader;
use std::{fs::File, io::Read};

use self::sections::WasmSections;

// pub use sections::WasmSections;

#[derive(Debug)]
pub struct WasmModule {
    sections: WasmSections,
}

impl WasmModule {
    pub fn from_file(file: &mut File) -> Result<WasmModule, WasmParserError> {
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        WasmModule::from_bytes(&buffer)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<WasmModule, WasmParserError> {
        let mut reader = ByteReader::from_vec(bytes);
        let Ok(_magic) = reader.read_expect(b"\0asm") else {
			return Err(WasmParserError::InvalidWasmBytes { message: "The first four bytes in an wasm file / byte buffer should start with '\\0asm' (0x00, 0x61, 0x73, 0x6D)" })
		};

        let Ok(_version) = reader.read_expect(&[0x01, 0x00, 0x00, 0x00]) else {
			return Err(WasmParserError::InvalidWasmBytes { message: "The bytes (4 through 7) should be the version number of the wasm binary and currently needs to be exactly (0x01, 0x00, 0x00, 0x00)" })
		};

        Ok(WasmModule {
            sections: WasmSections::from_reader(&mut reader)?,
        })
    }
}
