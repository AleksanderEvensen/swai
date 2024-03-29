use bytereader::{ByteReader, ByteReaderError, FromByteReader};

use crate::{error::WasmParserError, leb128::Leb128Readers, types::Indecies};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Instructions {
    // Control Instructions
    BlockType,      // 0x40
    Unreachable,    // 0x00
    Nop,            // 0x01
    Block,          // 0x02
    Loop,           // 0x03
    IfElse,         // 0x04  0x0B
    IfElseJmp,      // 0x04  0x05
    Br,             // 0x0C
    BrIf,           // 0x0D
    BrTable,        // 0x0E
    Return,         // 0x0F
    Call(Indecies), // 0x10
    CallIndirect,   // 0x11

    // Reference Instructions
    RefNull,   // 0xD0
    RefIsNull, // 0xD1
    RefFunc,   // 0xD2

    // Parametric Instructions
    Drop,           // 0x1A
    Select,         // 0x1B
    SelectMultiple, // 0x1C  - TODO: Double check this one

    // Variable Instructions
    LocalGet(Indecies), // 0x20
    LocalSet,           // 0x21
    LocalTee,           // 0x22
    GlobalGet,          // 0x23
    GlobalSet,          // 0x24

    // Table Instructions
    TableGet,  // 0x25
    TableSet,  // 0x26
    TableInit, // 0xFC 12
    ElemDrop,  // 0xFC 13
    TableCopy, // 0xFC 14
    TableGrow, // 0xFC 15
    TableSize, // 0xFC 16
    TableFill, // 0xFC 17

    // Memory Instructions
    i32_load,             // 0x28
    i64_load,             // 0x29
    f32_load,             // 0x2A
    f64_load,             // 0x2B
    i32_load_8s,          // 0x2C
    i32_load_8u,          // 0x2D
    i32_load_16s,         // 0x2E
    i32_load_16u,         // 0x2F
    i64_load_8s,          // 0x30
    i64_load_8u,          // 0x31
    i64_load_16s,         // 0x32
    i64_load_16u,         // 0x33
    i64_load_32s,         // 0x34
    i64_load_32u,         // 0x35
    i32_store,            // 0x36
    i64_store,            // 0x37
    f32_store,            // 0x38
    f64_store,            // 0x39
    i32_store_8,          // 0x3A
    i32_store_16,         // 0x3B
    i64_store_8,          // 0x3C
    i64_store_16,         // 0x3D
    i64_store_32,         // 0x3E
    MemorySize,           // 0x3F 0x00
    MemoryGrow,           // 0x40 0x00
    MemoryInit(Indecies), // 0xFC 8
    DataDrop,             // 0xFC 9
    MemoryCopy,           // 0xFC 10
    MemoryFill,           // 0xFC 11

    // Numeric Instructions
    i32_const(i32), // 0x41
    i64_const(i64), // 0x42
    f32_const(f32), // 0x43
    f64_const(f64), // 0x44

    i32_eqz,  // 0x45
    i32_eq,   // 0x46
    i32_ne,   // 0x47
    i32_lt_s, // 0x48
    i32_lt_u, // 0x49
    i32_gt_s, // 0x4A
    i32_gt_u, // 0x4B
    i32_le_s, // 0x4C
    i32_le_u, // 0x4D
    i32_ge_s, // 0x4E
    i32_ge_u, // 0x4F

    i64_eqz,  // 0x50
    i64_eq,   // 0x51
    i64_ne,   // 0x52
    i64_lt_s, // 0x53
    i64_lt_u, // 0x54
    i64_gt_s, // 0x55
    i64_gt_u, // 0x56
    i64_le_s, // 0x57
    i64_le_u, // 0x58
    i64_ge_s, // 0x59
    i64_ge_u, // 0x5A

    f32_eq, // 0x5B
    f32_ne, // 0x5C
    f32_lt, // 0x5D
    f32_gt, // 0x5E
    f32_le, // 0x5F
    f32_ge, // 0x60

    f64_eq, // 0x61
    f64_ne, // 0x62
    f64_lt, // 0x63
    f64_gt, // 0x64
    f64_le, // 0x65
    f64_ge, // 0x66

    i32_clz,    // 0x67
    i32_ctz,    // 0x68
    i32_popcnt, // 0x69
    i32_add,    // 0x6A
    i32_sub,    // 0x6B
    i32_mul,    // 0x6C
    i32_div_s,  // 0x6D
    i32_div_u,  // 0x6E
    i32_rem_s,  // 0x6F
    i32_rem_u,  // 0x70
    i32_and,    // 0x71
    i32_or,     // 0x72
    i32_xor,    // 0x73
    i32_shl,    // 0x74
    i32_shr_s,  // 0x75
    i32_shr_u,  // 0x76
    i32_rotl,   // 0x77
    i32_rotr,   // 0x78

    i64_clz,    // 0x79
    i64_ctz,    // 0x7A
    i64_popcnt, // 0x7B
    i64_add,    // 0x7C
    i64_sub,    // 0x7D
    i64_mul,    // 0x7E
    i64_div_s,  // 0x7F
    i64_div_u,  // 0x80
    i64_rem_s,  // 0x81
    i64_rem_u,  // 0x82
    i64_and,    // 0x83
    i64_or,     // 0x84
    i64_xor,    // 0x85
    i64_shl,    // 0x86
    i64_shr_s,  // 0x87
    i64_shr_u,  // 0x88
    i64_rotl,   // 0x89
    i64_rotr,   // 0x8A

    f32_abs,      // 0x8B
    f32_neg,      // 0x8C
    f32_ceil,     // 0x8D
    f32_floor,    // 0x8E
    f32_trunc,    // 0x8F
    f32_nearest,  // 0x90
    f32_sqrt,     // 0x91
    f32_add,      // 0x92
    f32_sub,      // 0x93
    f32_mul,      // 0x94
    f32_div,      // 0x95
    f32_min,      // 0x96
    f32_max,      // 0x97
    f32_copysign, // 0x98

    f64_abs,      // 0x99
    f64_neg,      // 0x9A
    f64_ceil,     // 0x9B
    f64_floor,    // 0x9C
    f64_trunc,    // 0x9D
    f64_nearest,  // 0x9E
    f64_sqrt,     // 0x9F
    f64_add,      // 0xA0
    f64_sub,      // 0xA1
    f64_mul,      // 0xA2
    f64_div,      // 0xA3
    f64_min,      // 0xA4
    f64_max,      // 0xA5
    f64_copysign, // 0xA6

    i32_wrap_i64,        // 0xA7
    i32_trunc_f32_s,     // 0xA8
    i32_trunc_f32_u,     // 0xA9
    i32_trunc_f64_s,     // 0xAA
    i32_trunc_f64_u,     // 0xAB
    i64_extend_i32_s,    // 0xAC
    i64_extend_i32_u,    // 0XAD
    i64_trunc_f32_s,     // 0xAE
    i64_trunc_f32_u,     // 0xAF
    i64_trunc_f64_s,     // 0xB0
    i64_trunc_f64_u,     // 0xB1
    f32_convert_i32_s,   // 0xB2
    f32_convert_i32_u,   // 0xB3
    f32_convert_i64_s,   // 0xB4
    f32_convert_i64_u,   // 0xB5
    f32_demote_f64,      // 0xB6
    f64_convert_i32_s,   // 0xB7
    f64_convert_i32_u,   // 0xB8
    f64_convert_i64_s,   // 0xB9
    f64_convert_i64_u,   // 0xBA
    f64_promote_f32,     // 0xBB
    i32_reinterpret_f32, // 0xBC
    i64_reinterpret_f64, // 0xBD
    f32_reinterpret_i32, // 0xBE
    f64_reinterpret_i64, // 0xBF

    i32_extend8_s,  // 0xC0
    i32_extend16_s, // 0xC1
    i64_extend8_s,  // 0xC2
    i64_extend16_s, // 0xC3
    i64_extend32_s, // 0xC4

    i32_trunc_sat_f32_s, // 0xFC 0
    i32_trunc_sat_f32_u, // 0xFC 1
    i32_trunc_sat_f64_s, // 0xFC 2
    i32_trunc_sat_f64_u, // 0xFC 3
    i64_trunc_sat_f32_s, // 0xFC 4
    i64_trunc_sat_f32_u, // 0xFC 5
    i64_trunc_sat_f64_s, // 0xFC 6
    i64_trunc_sat_f64_u, // 0xFC 7
}

impl FromByteReader for Instructions {
    fn read_from_byte_reader(
        reader: &mut bytereader::ByteReader,
    ) -> Result<Self, bytereader::ByteReaderError>
    where
        Self: Sized,
    {
        Ok(match reader.read::<u8>()? {
            0x10 => Instructions::Call(reader.read_uleb128::<u32>().map(Indecies::FuncIdx)?),
            0x20 => Instructions::LocalGet(reader.read_uleb128::<u32>().map(Indecies::LocalIdx)?),
            0x41 => Instructions::i32_const(reader.read_leb128::<i32>()?),
            0x6A => Instructions::i32_add,

            0xFC => match reader.read_uleb128::<u32>()? {
                8 => {
                    let v =
                        Instructions::MemoryInit(Indecies::DataIdx(reader.read_uleb128::<u32>()?));
                    reader.read_expect(&[0x00])?;
                    v
                }

                variant => {
                    return Err(bytereader::ByteReaderError::UnknownError(format!(
                        "Invalid instruction opcode variant for opcode '0xFC': '{}'",
                        variant
                    )))
                }
            },

            opcode_id => {
                return Err(bytereader::ByteReaderError::UnknownError(format!(
                    "Invalid instruction opcode id: '0x{:X?}'",
                    opcode_id
                )))
            }
        })
    }
}

pub fn read_expr(reader: &mut ByteReader) -> Result<Vec<Instructions>, ByteReaderError> {
    let mut opcodes = vec![];

    while let Some(byte) = reader.peak::<u8>().ok() {
        if byte == 0x0B {
            break;
        } // End of expression
        opcodes.push(reader.read::<Instructions>()?);
    }
    reader.jump(1); // jump over the 0x0B escape byte
    return Ok(opcodes);
}
