#![allow(non_camel_case_types, unused, non_snake_case)]

use std::ops::{RangeFrom, RangeInclusive};

use nom::{
    bytes::complete::tag,
    combinator::map,
    error::{ContextError, ParseError},
    number::complete::u8,
    sequence::tuple,
    IResult, InputIter, InputLength, Parser, Slice,
};
use nom_leb128::leb128_u32;

#[derive(Debug)]
pub enum Indecies {
    TypeIdx(u32),
    FuncIdx(u32),
    TableIdx(u32),
    MemIdx(u32),
    GlobalIdx(u32),
    ElemIdx(u32),
    DataIdx(u32),
    LocalIdx(u32),
    LabelIdx(u32),
}

#[derive(Debug)]
pub struct Name(String);

impl Name {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Name> {
        let (input, bytes) = vec(u8)(input)?;

        let str = match String::from_utf8(bytes) {
            Ok(v) => v,
            Err(e) => {
                panic!("Something went wrong when converting utf8 bytes to string: {e}")
            }
        };

        Ok((input, Name(str)))
    }
}

#[derive(Debug)]
pub enum NumberTypes {
    i32,
    i64,
    f32,
    f64,
}
impl From<u8> for NumberTypes {
    fn from(value: u8) -> Self {
        match value {
            0x7F => NumberTypes::i32,
            0x7E => NumberTypes::i64,
            0x7D => NumberTypes::f32,
            0x7C => NumberTypes::f64,
            _ => unreachable!("Check the wasm spec for more info: https://webassembly.github.io/spec/core/binary/types.html#number-types"),
        }
    }
}

#[derive(Debug)]
pub enum VectorTypes {
    v128,
}

impl From<u8> for VectorTypes {
    fn from(value: u8) -> Self {
        match value {
            0x7B => VectorTypes::v128,
			_ => unreachable!("Check the wasm spec for more info: https://webassembly.github.io/spec/core/binary/types.html#vector-types")
        }
    }
}

#[derive(Debug)]
pub enum ReferenceTypes {
    funcref,
    externref,
}

impl From<u8> for ReferenceTypes {
    fn from(value: u8) -> Self {
        match value {
			0x70 => ReferenceTypes::funcref,
			0x6F => ReferenceTypes::externref,
			_ => unreachable!("Check the wasm spec for more info: https://webassembly.github.io/spec/core/binary/types.html#reference-types")
		}
    }
}

#[derive(Debug)]
pub enum ValueType {
    NumType(NumberTypes),
    VecType(VectorTypes),
    RefType(ReferenceTypes),
}

impl From<u8> for ValueType {
    fn from(value: u8) -> Self {
        match value {
            0x7C..=0x7F => ValueType::NumType(NumberTypes::from(value)),
            0x7B => ValueType::VecType(VectorTypes::from(value)),
            0x6F..=0x70 => ValueType::RefType(ReferenceTypes::from(value)),
			_ => unreachable!("Check the wasm spec for more info: https://webassembly.github.io/spec/core/binary/types.html#value-types")
        }
    }
}

impl ValueType {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], ValueType> {
        map(u8, |byte| ValueType::from(byte))(input)
    }
}

pub(crate) mod ResultType {
    use super::{vec, ValueType};
    use nom::IResult;

    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Vec<ValueType>> {
        vec(ValueType::parse)(input)
    }
}

#[derive(Debug)]
pub struct FunctionType {
    params: Vec<ValueType>,
    result: Vec<ValueType>,
}

impl FunctionType {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((tag(&[0x60]), ResultType::parse, ResultType::parse)),
            |(_, params, result)| FunctionType { params, result },
        )(input)
    }
}

#[derive(Debug)]
pub enum Limits {
    min(RangeFrom<u32>),
    minmax(RangeInclusive<u32>),
}

impl Limits {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (limit_type, n)) = tuple((u8, leb128_u32))(input)?;

        match limit_type {
            0x00 => Ok((input, Limits::min(n..))),
            0x01 => {
                let (input, m) = leb128_u32(input)?;
                Ok((input, Limits::minmax(n..=m)))
            }
			_ => unreachable!("Check the wasm spec for more info: https://webassembly.github.io/spec/core/binary/types.html#limits")
        }
    }
}

pub(crate) mod MemoryType {
    use super::Limits;
    use nom::IResult;

    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Limits> {
        Limits::parse(input)
    }
}

#[derive(Debug)]
pub struct TableType {
    elem: ReferenceTypes,
    lim: Limits,
}

impl TableType {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], TableType> {
        map(tuple((u8, Limits::parse)), |(reftype_val, lim)| TableType {
            elem: ReferenceTypes::from(reftype_val),
            lim,
        })(input)
    }
}

#[derive(Debug)]
pub struct GlobalType {
    vtype: ValueType,
    mutability: Mutability,
}

impl GlobalType {
    pub(crate) fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((ValueType::parse, u8)), |(vtype, mutability)| {
            GlobalType {
                vtype,
                mutability: Mutability::from(mutability),
            }
        })(input)
    }
}

#[derive(Debug)]
pub enum Mutability {
    Const,
    Var,
}

impl From<u8> for Mutability {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Mutability::Const,
            0x01 => Mutability::Var,
			_ => unreachable!("Check the wasm spec for more info: https://webassembly.github.io/spec/core/binary/types.html#global-types")
        }
    }
}

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
