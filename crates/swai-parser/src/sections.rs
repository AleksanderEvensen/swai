use bytereader::ByteReader;

use super::types::FunctionType;
use crate::{
    error::WasmParserError,
    leb128::Leb128Readers,
    types::{read_vec, Indecies, Name, ValueType, ImportDesc, TableType, MemType}, instructions::Instructions,
};

#[derive(Debug)]
pub struct WasmSections {
    pub custom: Vec<()>,
    pub types: Vec<FunctionType>,
    pub imports: Vec<(Name, Name, ImportDesc)>,
    pub functions: Vec<Indecies>,
    pub tables: Vec<TableType>,
    pub memory: Vec<MemType>,
    pub global: Vec<()>,
    pub export: Vec<(Name, Indecies)>,
    pub start: Option<()>,
    pub element: Vec<()>,
    pub code: Vec<(Vec<(u32, ValueType)>, Vec<Instructions>)>,
    pub data: Vec<()>,
    pub data_count: Option<u32>,
}

impl WasmSections {
    pub fn from_reader(reader: &mut ByteReader) -> Result<Self, WasmParserError> {
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

        while let Ok(section_id) = reader.read::<u8>() {
            let _section_size = reader.read_uleb128::<u32>()?;

            println!("Id: {section_id}  |  Size: {_section_size}");
            match section_id {
                0 => {
                    reader.jump(_section_size as usize);
                    println!("TODO: implement custom section");
                }
                1 => sections.types = read_vec(reader)?,
                2 => sections.imports = (0..reader.read_uleb128::<u32>()?).map(|_| Ok((reader.read::<Name>()?, reader.read::<Name>()?, reader.read::<ImportDesc>()?))).collect::<Result<_,WasmParserError>>()?,
                3 => {
                    sections.functions = (0..reader.read_uleb128::<u32>()?)
                        .map(|_| reader.read_uleb128::<u32>().map(Indecies::TypeIdx))
                        .collect::<Result<_, _>>()?;
                }
                4 => sections.tables = read_vec(reader)?,
                5 => sections.memory = read_vec(reader)?,
                6 => todo!("global section"),
                7 => {
                    sections.export = (0..reader.read_uleb128::<u32>()?)
                        .map(|_| {
                            Ok((
                                reader.read()?,
                                match reader.read::<u8>()? {
                                    0x00 => reader.read_uleb128::<u32>().map(Indecies::FuncIdx)?,
                                    0x01 => reader.read_uleb128::<u32>().map(Indecies::TableIdx)?,
                                    0x02 => reader.read_uleb128::<u32>().map(Indecies::MemIdx)?,
                                    0x03 => {
                                        reader.read_uleb128::<u32>().map(Indecies::GlobalIdx)?
                                    }
                                    id => {
                                        return Err(WasmParserError::InvalidSectionError {
                                            message: format!(
                                                "Export section had an invalid (Name, Index) pair. The provided index ({}) isn't valid", id
											),
                                        })
                                    }
                                },
                            ))
                        })
                        .collect::<Result<_, _>>()?
                }
                8 => todo!("start section"),
                9 => todo!("element section"),
                10 => {
					
					sections.code = (0..reader.read_uleb128::<u32>()?).map(|_| {
						let code_sec_bytes = reader.read_uleb128::<u32>()?;
						let bytes_end = reader.get_current_offset() + code_sec_bytes as usize;
						let locals = (0..reader.read_uleb128::<u32>()?).map(|_| Ok((reader.read()?, reader.read()?))).collect::<Result<Vec<(u32, ValueType)>, WasmParserError>>()?;
						let code_bytes = reader.read_bytes(bytes_end - reader.get_current_offset())?.to_vec();
						
						let mut code_reader = ByteReader::from_vec(&code_bytes);
						let mut opcodes = vec![];

						while let Some(byte) = code_reader.peak::<u8>().ok() {
							if byte == 0x0B { break } // End of expression
							opcodes.push(code_reader.read::<Instructions>()?);
						}

						println!("Opcodes:\n{:#?}", opcodes);


						Ok((locals, opcodes))
					}).collect::<Result<_,WasmParserError>>()?;

				},
                11 => todo!("data section"),
                12 => todo!("data_count section"),

                id => return Err(WasmParserError::InvalidSectionId { id }),
            }
        }

        Ok(sections)
    }
}
