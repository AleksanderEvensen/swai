use nom::{
    bytes::complete::{tag, take},
    combinator::map,
    multi::fold_many0,
    number::complete::u8,
    sequence::tuple,
    IResult,
};
use nom_leb128::leb128_u32;
use std::{
    fs::File,
    io::{self, Read},
};
use thiserror::Error;
use wasm_parsers::ValueType;

use crate::wasm_parsers::{valtype, vec};

mod wasm_parsers;

#[derive(Error, Debug)]
pub enum WasmParseError {
    #[error("IO Error occurred")]
    IoError(#[from] io::Error),

    #[error("Unrecoverable parser error occurred")]
    NomError,
}

pub fn parse(file: &mut File) -> Result<(), WasmParseError> {
    let mut input = vec![];
    file.read_to_end(&mut input)?;

    let (_, output) = parse_bytes(&input[..]).map_err(|_| WasmParseError::NomError)?;

    Ok(output)
}

pub enum WasmSection {}

fn parse_bytes(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _magic) = tag("\0asm")(input)?;
    let (input, _version) = tag(&[0x01, 0x00, 0x00, 0x00])(input)?;

    let (input, sections) = fold_many0(parse_section, Vec::new, |mut acc, item| {
        acc.push(item);
        acc
    })(input)?;

    println!("{:#?}", sections);

    Ok((input, ()))
}

fn parse_section(input: &[u8]) -> IResult<&[u8], Section> {
    let (input, (section_id, section_size)) = tuple((u8, leb128_u32))(input)?;

    let (input, section_bytes) = take(section_size)(input)?;

    println!("SectionId: {section_id}  |  SectionSize: {section_size}");
    let (_, section) = match section_id {
        0 => todo!("Implement custom section"),
        1 => map(
            vec(map(
                tuple((tag(&[0x60]), vec(valtype), vec(valtype))),
                |(_, params_types, result_types)| (params_types, result_types),
            )),
            |functions_types| Section::TypeSection(functions_types),
        )(section_bytes)?,
        2 => todo!("Implement import section"),
        3 => todo!("Implement function section"),
        4 => todo!("Implement table section"),
        5 => todo!("Implement memory section"),
        6 => todo!("Implement global section"),
        7 => todo!("Implement export section"),
        8 => todo!("Implement start section"),
        9 => todo!("Implement element section"),
        10 => todo!("Implement code section"),
        11 => todo!("Implement data section"),
        12 => todo!("Implement data count section"),

        unknown_section_id => todo!("Unrecognized section: {unknown_section_id}"),
    };

    Ok((input, section))
}

#[derive(Debug)]
pub enum Section {
    TypeSection(Vec<(Vec<ValueType>, Vec<ValueType>)>),
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::parse;

    #[test]
    fn parse_add_test() -> std::io::Result<()> {
        let mut file = File::open("tests/add.wasm")?;

        parse(&mut file).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to parse the file 'tests/add.wasm'",
            )
        })?;

        Ok(())
    }
}
