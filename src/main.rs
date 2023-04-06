use std::fs::File;

use nom::{IResult, InputIter};

fn main() {
    let mut add_file = File::open("./tests/memory.wasm").unwrap();
    let data = swai::parse(&mut add_file).unwrap();

    println!("{:#?}", data);

    let input: &[u8] = &[0xBF, 0x80, 0x80, 0x80, 0x80];

    println!("{:?}", test(input));

    println!("{}", leb128_u64(input));
}

fn test(input: &[u8]) -> IResult<&[u8], u64> {
    nom_leb128::leb128_u64(input)
}

fn leb128_u64(input: &[u8]) -> u64 {
    let mut result: u64 = 0;
    let mut shift: u64 = 0;

    for (_, byte) in input.iter_indices() {
        result |= ((byte & 0x7F) as u64) << shift;

        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;
    }

    return result;
}
