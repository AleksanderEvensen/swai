#![allow(non_snake_case)]
use nom::{bytes::complete::take, number::complete::u8, sequence::tuple, IResult};
use nom_leb128::leb128_u32;

pub mod Code;
pub mod Custom;
pub mod Data;
pub mod Element;
pub mod Export;
pub mod Function;
pub mod Global;
pub mod Import;
pub mod Memory;
pub mod Start;
pub mod Table;
pub mod Type;

#[derive(Debug)]
pub enum Section {
    CodeSection,
    CustomSection,
    DataSection,
    ElementSection,
    ExportSection,
    FunctionSection,
    GlobalSection,
    ImportSection,
    MemorySection,
    StartSection,
    TableSection,
    TypeSection,
}

impl Section {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Option<Section>> {
        let (input, (section_id, section_size)) = tuple((u8, leb128_u32))(input)?;
        let (input, section_bytes) = take(section_size)(input)?;

        println!("SectionId: {section_id}  |  SectionSize: {section_size}");
        Ok((input, match section_id {
            0 => None, // Custom Section not implemented yet because it isn't neccessary
            1 => todo!("Type Section"),
            2 => todo!("Import Section"),
            3 => todo!("Function Section"),
            4 => todo!("Table Section"),
            5 => todo!("Memory Section"),
            6 => todo!("Global Section"),
            7 => todo!("Export Section"),
            8 => todo!("Start Section"),
            9 => todo!("Element Section"),
            10 => todo!("Code Section"),
            11 => todo!("Data Section"),
            12 => todo!("DataCount Section"),
			_ => unreachable!("Check the wasm spec for more info: https://webassembly.github.io/spec/core/binary/modules.html#sections")
        }))
    }
}
