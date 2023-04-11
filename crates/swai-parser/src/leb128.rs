use bytereader::{ByteReader, ByteReaderError};
use std::ops::{BitOrAssign, Shl};

pub trait Leb128Readers {
    fn read_uleb128<T>(&mut self) -> Result<T, ByteReaderError>
    where
        T: Sized + Shl<usize, Output = T> + BitOrAssign + From<u8>;
    fn read_leb128<T>(&mut self) -> Result<T, ByteReaderError>
    where
        T: Sized + Shl<usize, Output = T> + BitOrAssign + From<u8> + From<i32>;

    fn peak_uleb128<T>(&mut self) -> Result<T, ByteReaderError>
    where
        T: Sized + Shl<usize, Output = T> + BitOrAssign + From<u8>;
    fn peak_leb128<T>(&mut self) -> Result<T, ByteReaderError>
    where
        T: Sized + Shl<usize, Output = T> + BitOrAssign + From<u8> + From<i32>;
}

impl Leb128Readers for ByteReader {
    fn read_uleb128<T>(&mut self) -> Result<T, ByteReaderError>
    where
        T: Sized + Shl<usize, Output = T> + BitOrAssign + From<u8>,
    {
        let mut result: T = T::from(0);
        let mut shift = 0;

        let mut i = 0;
        while let Ok(byte) = self.read::<u8>() {
            result |= T::from(byte & 0x7F) << shift;

            if (byte & 0x80) == 0 {
                return Ok(result);
            } else if i >= leb128_size::<Self>() - 1 {
                return Err(ByteReaderError::UnknownError(
                    "Number is too large (Integer overflow)".to_string(),
                ));
            }

            shift += 7;
            i += 1;
        }

        Err(ByteReaderError::UnknownError(
            "Incomplete byte array".to_string(),
        ))
    }

    fn read_leb128<T>(&mut self) -> Result<T, ByteReaderError>
    where
        T: Sized + Shl<usize, Output = T> + BitOrAssign + From<u8> + From<i32>,
    {
        let mut result: T = T::from(0);
        let mut shift = 0;
        let size = core::mem::size_of::<i64>() * 8;

        let mut i = 0;

        while let Ok(byte) = self.read::<u8>() {
            result |= T::from(byte & 0x7F) << shift;
            shift += 7;

            if (byte & 0x80) == 0 {
                if (shift < size) && (byte & 0x40) != 0 {
                    result |= T::from(!0 << shift);
                }

                return Ok(result);
            } else if i >= leb128_size::<u64>() - 1 {
                return Err(ByteReaderError::UnknownError(
                    "Number is too large (Integer overflow)".to_string(),
                ));
            }

            i += 1;
        }

        Err(ByteReaderError::UnknownError(
            "Incomplete byte array".to_string(),
        ))
    }

    fn peak_uleb128<T>(&mut self) -> Result<T, ByteReaderError>
    where
        T: Sized + Shl<usize, Output = T> + BitOrAssign + From<u8>,
    {
        self.push_index();
        let v = self.read_uleb128()?;
        self.pop_index();
        Ok(v)
    }

    fn peak_leb128<T>(&mut self) -> Result<T, ByteReaderError>
    where
        T: Sized + Shl<usize, Output = T> + BitOrAssign + From<u8> + From<i32>,
    {
        self.push_index();
        let v = self.read_leb128()?;
        self.pop_index();
        Ok(v)
    }
}

// Thanks to the nom-leb128 crate for the size determin function: https://github.com/milkey-mouse/nom-leb128/blob/58f37d293eeb4d43f44a38650802b1defda607c3/src/lib.rs#L17-L20
fn leb128_size<T>() -> usize {
    let bits = std::mem::size_of::<T>() * 8;
    (bits + 6) / 7
}
