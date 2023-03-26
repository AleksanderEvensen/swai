use std::ops::RangeFrom;

use nom::{
    error::{ContextError, ParseError},
    number::complete::u8,
    IResult, InputIter, InputLength, Parser, Slice,
};
use nom_leb128::leb128_u32;

pub(crate) fn vec<I, O, E, F>(mut parser: F) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    F: Parser<I, O, E>,
    I: Clone + Slice<RangeFrom<usize>> + InputIter<Item = u8> + InputLength,
    E: ParseError<I> + ContextError<I>,
{
    move |input: I| {
        let (mut input, n) = leb128_u32(input)?;
        let mut vec: Vec<O> = vec![];

        for _ in 0..n {
            let (inp, val) = parser.parse(input)?;
            input = inp;
            vec.push(val);
        }

        Ok((input, vec))
    }
}

pub(crate) fn valtype(input: &[u8]) -> IResult<&[u8], ValueType> {
    let (input, vtype) = u8(input)?;

    let value_type = match vtype {
		0x7F => ValueType::NumType(NumberTypes::i32),
		0x7E => ValueType::NumType(NumberTypes::i64),
		0x7D => ValueType::NumType(NumberTypes::f32),
		0x7C => ValueType::NumType(NumberTypes::f64),

		0x7B => ValueType::VecType,

		0x70 => ValueType::RefType(ReferenceTypes::funcref),
		0x6F => ValueType::RefType(ReferenceTypes::externref),

		unknown_value_type => unreachable!("Should be unreachable, check the WebAssembly reference for information on new Value Types") 
	};

    Ok((input, value_type))
}

#[derive(Debug)]
pub enum ValueType {
    NumType(NumberTypes),
    VecType,
    RefType(ReferenceTypes),
}

#[derive(Debug)]
pub enum NumberTypes {
    i32,
    i64,

    f32,
    f64,
}

#[derive(Debug)]
pub enum ReferenceTypes {
    funcref,
    externref,
}
