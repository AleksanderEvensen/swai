pub mod sections;
pub mod types;

use crate::error::WasmParserError;
use crate::leb128::Leb128Readers;
use bytereader::ByteReader;
use std::{fs::File, io::Read};

pub use sections::WasmSections;

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
        let mut sections = WasmSections {
            custom: vec![],
            types: vec![],
            imports: vec![],
            functions: vec![],
            tables: vec![],
            memory: vec![],
            global: vec![],
            export: vec![],
            start: None,
            element: vec![],
            code: vec![],
            data: vec![],
            data_count: None,
        };

        let Ok(_magic) = reader.read_expect(b"\0asm") else {
			return Err(WasmParserError::InvalidWasmBytes { message: "The first four bytes in an wasm file / byte buffer should start with '\\0asm' (0x00, 0x61, 0x73, 0x6D)" })
		};

        let Ok(_version) = reader.read_expect(&[0x01, 0x00, 0x00, 0x00]) else {
			return Err(WasmParserError::InvalidWasmBytes { message: "The bytes (4 through 7) should be the version number of the wasm binary and currently needs to be exactly (0x01, 0x00, 0x00, 0x00)" })
		};

        while let Ok(section_id) = reader.read::<u8>() {
            let _section_size = reader.read_uleb128::<u32>()?;

            println!("Id: {section_id}  |  Size: {_section_size}");
            reader.jump(_section_size as usize);
            continue;
            match section_id {
                0 => todo!("custom section"),
                1 => todo!("type section"),
                2 => todo!("import section"),
                3 => todo!("function section"),
                4 => todo!("table section"),
                5 => todo!("memory section"),
                6 => todo!("global section"),
                7 => todo!("export section"),
                8 => todo!("start section"),
                9 => todo!("element section"),
                10 => todo!("code section"),
                11 => todo!("data section"),
                12 => todo!("data_count section"),

                id => return Err(WasmParserError::InvalidSectionId { id }),
            }
        }

        Ok(WasmModule { sections })
    }
}
