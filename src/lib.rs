use nom::{bytes::complete::tag, multi::fold_many0, IResult};
use std::{
    fs::File,
    io::{self, Read},
};
use thiserror::Error;

use crate::wasm::Sections::Section;

mod error;
pub mod wasm;

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

fn parse_bytes(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _magic) = tag("\0asm")(input)?;
    let (input, _version) = tag(&[0x01, 0x00, 0x00, 0x00])(input)?;

    let (input, sections) = fold_many0(Section::parse, Vec::new, |mut acc, item| {
        acc.push(item);
        acc
    })(input)?;

    println!("{:#?}", sections);

    Ok((input, ()))
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
