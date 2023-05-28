use bytereader::ByteReader;

use super::types::FunctionType;
use crate::{
    error::WasmParserError,
    leb128::Leb128Readers,
    types::{read_vec, Indecies, Name},
};

#[derive(Debug)]
pub struct WasmSections {
    pub custom: Vec<()>,
    pub types: Vec<FunctionType>,
    pub imports: Vec<()>,
    pub functions: Vec<Indecies>,
    pub tables: Vec<()>,
    pub memory: Vec<()>,
    pub global: Vec<()>,
    pub export: Vec<(Name, Indecies)>,
    pub start: Option<()>,
    pub element: Vec<()>,
    pub code: Vec<()>,
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
                2 => todo!("import section"),
                3 => {
                    sections.functions = (0..reader.read_uleb128::<u32>()?)
                        .map(|_| reader.read_uleb128::<u32>().map(Indecies::TypeIdx))
                        .collect::<Result<_, _>>()?;
                }
                4 => todo!("table section"),
                5 => todo!("memory section"),
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
                10 => todo!("code section"),
                11 => todo!("data section"),
                12 => todo!("data_count section"),

                id => return Err(WasmParserError::InvalidSectionId { id }),
            }
        }

        Ok(sections)
    }
}

/*
#[derive(Debug)]
pub enum Section {
    CodeSection,
    CustomSection,
    DataSection,
    ElementSection,
    ExportSection(Vec<(Name, Indecies)>),
    FunctionSection(Vec<Indecies>),
    GlobalSection,
    ImportSection,
    MemorySection(Vec<Vec<ValueType>>),
    StartSection,
    TableSection,
    TypeSection(Vec<FunctionType>),
}

impl Section {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Option<Section>> {
        // let (input, (section_id, section_size)) = tuple((u8, leb128_u32))(input)?;
        // let (input, section_bytes) = take(section_size)(input)?;

        println!("SectionId: {section_id}  |  SectionSize: {section_size}");
        Ok((input, match section_id {
            0 => None, // Custom Section not implemented yet because it isn't neccessary
            1 => {
                if let Ok((_, section)) = TypeSection::parse(section_bytes) {
                    Some(Section::TypeSection(section))
                } else {
                    None
                }
            },
            2 => todo!("Import Section"),
            3 => {
                if let Ok((_, section)) = FunctionSection::parse(section_bytes) {
                    Some(Section::FunctionSection(section))
                } else {
                    None
                }
            },
            4 => todo!("Table Section"),
            5 => {
                if let Ok((_, section)) = MemorySection::parse(section_bytes) {
                    Some(Section::MemorySection(section))
                } else {
                    None
                }
            },
            6 => todo!("Global Section"),
            7 => {
                if let Ok((_, section)) = ExportSection::parse(section_bytes) {
                    Some(Section::ExportSection(section))
                } else {
                    None
                }
            },
            8 => todo!("Start Section"),
            9 => todo!("Element Section"),
            10 => {
                if let Ok((_, _)) = CodeSection::parse(section_bytes) {
                    Some(Section::CodeSection)
                } else {
                    None
                }
            },
            11 => todo!("Data Section"),
            12 => todo!("DataCount Section"),
            _ => unreachable!("Check the wasm spec for more info: https://webassembly.github.io/spec/core/binary/modules.html#sections")
        }))
    }
}

mod FunctionSection {
    use crate::wasm::types::{vec, Indecies};
    use nom::{combinator::map, IResult};
    use nom_leb128::leb128_u32;

    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Vec<Indecies>> {
        println!("{:?}", input);
        vec(map(leb128_u32, |idx| Indecies::TypeIdx(idx)))(input)
    }
}

mod ExportSection {
    use crate::wasm::types::{vec, Indecies, Name};
    use nom::{combinator::map, number::complete::u8, sequence::tuple, IResult};
    use nom_leb128::leb128_u32;

    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Vec<(Name, Indecies)>> {
        vec(tuple((
            Name::parse,
            map(tuple((u8, leb128_u32)), |(id_type, idx)| {
                match id_type {
                    0x00 => Indecies::FuncIdx(idx),
                    0x01 => Indecies::TableIdx(idx),
                    0x02 => Indecies::MemIdx(idx),
                    0x03 => Indecies::GlobalIdx(idx),
                    _ => unreachable!("Failed to parse Indecie for Export section check wasm spec for more info: https://webassembly.github.io/spec/core/binary/modules.html#export-section")
                }
            }),
        )))(input)
    }
}

mod CodeSection {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], ()> {
        let (mut input, vec_len) = leb128_u32(input)?;

        for _ in 0..vec_len {
            let (inp, size) = leb128_u32(input)?;
            let (inp, _) = take(size)(inp)?;
            input = inp;
        }
        Ok((input, ()))
    }
}

mod MemorySection {
    use crate::wasm::types::{vec, ResultType, ValueType};

    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Vec<Vec<ValueType>>> {
        vec(ResultType::parse)(input)
    }
}

' */
