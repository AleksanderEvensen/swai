#![allow(non_camel_case_types, unused, non_snake_case)]
use crate::{
    instructions::{read_expr, Instructions},
    leb128::Leb128Readers,
};
use bytereader::{ByteReader, ByteReaderError, FromByteReader};
use std::ops::{RangeFrom, RangeInclusive};

pub type MemType = Limits;
pub type Expr = Vec<Instructions>;

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

impl FromByteReader for Name {
    fn read_from_byte_reader(
        reader: &mut bytereader::ByteReader,
    ) -> Result<Self, bytereader::ByteReaderError>
    where
        Self: Sized,
    {
        let str_len = reader.read_uleb128::<u32>()?;
        let str = reader.read_string(str_len as usize)?;

        Ok(Self(str))
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

impl FromByteReader for ValueType {
    fn read_from_byte_reader(
        reader: &mut bytereader::ByteReader,
    ) -> Result<Self, bytereader::ByteReaderError>
    where
        Self: Sized,
    {
        Ok(Self::from(reader.read::<u8>()?))
    }
}

#[derive(Debug)]
pub struct FunctionType {
    params: Vec<ValueType>,
    result: Vec<ValueType>,
}

impl FromByteReader for FunctionType {
    fn read_from_byte_reader(
        reader: &mut bytereader::ByteReader,
    ) -> Result<Self, bytereader::ByteReaderError>
    where
        Self: Sized,
    {
        reader.read_expect(&[0x60])?;

        Ok(Self {
            params: read_vec(reader)?,
            result: read_vec(reader)?,
        })
    }
}

#[derive(Debug)]
pub enum Limits {
    min(RangeFrom<u32>),
    minmax(RangeInclusive<u32>),
}

impl FromByteReader for Limits {
    fn read_from_byte_reader(
        reader: &mut bytereader::ByteReader,
    ) -> Result<Self, bytereader::ByteReaderError>
    where
        Self: Sized,
    {
        let limit_type = reader.read::<u8>()?;
        let n = reader.read_uleb128::<u32>()?;

        match limit_type {
            0x00 => Ok(Self::min(n..)),
            0x01 => {
                let m = reader.read_uleb128::<u32>()?;
                Ok((Self::minmax(n..=m)))
            }

            v => Err(bytereader::ByteReaderError::UnknownError(format!(
                "Failed to parse Limits: limit_type must be either '0x00' or '0x01' got '0x{v:X?}'"
            ))),
        }
    }
}

#[derive(Debug)]
pub struct TableType {
    elem: ReferenceTypes,
    lim: Limits,
}

impl FromByteReader for TableType {
    fn read_from_byte_reader(
        reader: &mut bytereader::ByteReader,
    ) -> Result<Self, bytereader::ByteReaderError>
    where
        Self: Sized,
    {
        Ok(TableType {
            elem: ReferenceTypes::from(reader.read::<u8>()?),
            lim: reader.read()?,
        })
    }
}

#[derive(Debug)]
pub struct GlobalType {
    vtype: ValueType,
    mutability: Mutability,
}

impl FromByteReader for GlobalType {
    fn read_from_byte_reader(
        reader: &mut bytereader::ByteReader,
    ) -> Result<Self, bytereader::ByteReaderError>
    where
        Self: Sized,
    {
        Ok(GlobalType {
            vtype: reader.read()?,
            mutability: Mutability::from(reader.read::<u8>()?),
        })
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

pub fn read_vec<T>(reader: &mut ByteReader) -> Result<Vec<T>, ByteReaderError>
where
    T: FromByteReader,
{
    (0..reader.read_uleb128::<u32>()?)
        .map(|_| T::read_from_byte_reader(reader))
        .collect()
}

#[derive(Debug)]
pub enum ImportDesc {
    TypeIdx(Indecies),
    TableType(TableType),
    MemType(MemType),
    GlobalType(GlobalType),
}

impl FromByteReader for ImportDesc {
    fn read_from_byte_reader(reader: &mut ByteReader) -> Result<Self, ByteReaderError>
    where
        Self: Sized,
    {
        Ok(match reader.read::<u8>()? {
            0x00 => ImportDesc::TypeIdx(reader.read_uleb128::<u32>().map(Indecies::TypeIdx)?),
            0x01 => ImportDesc::TableType(reader.read()?),
            0x02 => ImportDesc::MemType(reader.read()?),
            0x03 => ImportDesc::GlobalType(reader.read()?),
			_ => unreachable!("Check the wasm spec for more info: https://webassembly.github.io/spec/core/binary/modules.html#binary-importsec")

        })
    }
}

#[derive(Debug)]
pub struct DataSegment {
    mode: SegmentMode,
    bytes: Vec<u8>,
}
#[derive(Debug)]
pub enum SegmentMode {
    Passive,
    Active { memory_index: u32, offset: Expr },
}

impl FromByteReader for DataSegment {
    fn read_from_byte_reader(reader: &mut ByteReader) -> Result<Self, ByteReaderError>
    where
        Self: Sized,
    {
        let bitfield = reader.read_uleb128::<u32>()?;
        Ok(DataSegment {
            mode: if bitfield & 0b01 == 0 {
                SegmentMode::Active {
                    offset: read_expr(reader)?,
                    memory_index: if bitfield == 2 {
                        reader.read_uleb128::<u32>()?
                    } else {
                        0
                    },
                }
            } else {
                SegmentMode::Passive
            },
            bytes: read_vec::<u8>(reader)?,
        })
    }
}
