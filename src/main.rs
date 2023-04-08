use std::fs::File;

use nom::{IResult, InputIter};

fn main() {
    let mut add_file = File::open("./tests/memory.wasm").unwrap();
    let data = swai_parser::parse(&mut add_file).unwrap();

    println!("{:#?}", data);

    let input: &[u8] = &[0xBF, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x30];

    let input2: &[u8] = &[0x80, 0x80, 0x40];

    println!("Unsigned: {:?}", test(input));
    println!("Signed: {:?}", test2(input2));
    println!("---");

    println!("Unsigned: {:?}", leb128_u64(input));
    println!("Signed: {:?}", leb128_i64(input2));
}

fn test(input: &[u8]) -> IResult<&[u8], u64> {
    nom_leb128::leb128_u64(input)
}
fn test2(input: &[u8]) -> IResult<&[u8], i64> {
    nom_leb128::leb128_i64(input)
}

fn leb128_u64(input: &[u8]) -> Result<u64, String> {
    let mut result: u64 = 0;
    let mut shift: u64 = 0;

    for (i, byte) in input.iter_indices() {
        result |= ((byte & 0x7F) as u64) << shift;

        if (byte & 0x80) == 0 {
            return Ok(result);
        } else if i >= leb128_size::<u64>() - 1 {
            return Err("Number has is too large (Integer overflow)".to_string());
        }
        shift += 7;
    }
    Err(format!("Incomplete byte array"))
}

fn leb128_i64(input: &[u8]) -> Result<i64, String> {
    let mut result = 0;
    let mut shift = 0;
    let size = core::mem::size_of::<i64>() * 8;

    for (i, byte) in input.iter_indices() {
        result |= ((byte & 0x7F) as i64) << shift;
        shift += 7;

        if (byte & 0x80) == 0 {
            if (shift < size) && (byte & 0x40) != 0 {
                result |= !0 << shift;
            }

            return Ok(result);
        } else if i >= leb128_size::<u64>() - 1 {
            return Err("Number has is too large (Integer overflow)".to_string());
        }
    }

    Err(format!("Incomplete byte array"))
}

fn leb128_size<T>() -> usize {
    let bits = core::mem::size_of::<T>() * 8;
    (bits + 6) / 7
}
