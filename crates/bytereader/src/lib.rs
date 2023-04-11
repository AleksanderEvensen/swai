use std::{fs::File, io::Read, string::FromUtf8Error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ByteReaderError {
    #[error(
        "Attempted to read out of bounds of byte buffer (len: {length}) at offset: ({start}:{end})"
    )]
    OutOfBounds {
        length: usize,
        start: usize,
        end: usize,
    },
    #[error("Sequence ({sequence:?}) wasn't found in the byte buffer")]
    NotFound { sequence: Vec<u8> },

    #[error("Failed to cast  type '{from}' to '{to}'")]
    FailedTypeCast { from: String, to: String },

    #[error("Failed to parse Utf8String from bytes")]
    Utf8Error(#[from] FromUtf8Error),

    #[error("An unknown error occurred: '{0}'")]
    UnknownError(String),
}

#[derive(Clone, Copy)]
pub enum Endian {
    Little,
    Big,
}

#[derive(Clone)]
pub struct ByteReader {
    /// The byte buyffer
    data: Vec<u8>,
    /// The current offset (position) in the buffer
    offset: usize,
    /// Endian to use when reading for reading numbers
    endian: Endian,
    /// If the push_index function has been ran, the Option contains a value at that position, and can be returned to this position later using the pop_index function
    push_offsets: Vec<usize>,

    /// If needed enable this for verbose printing
    debug: bool,
}

impl ByteReader {
    pub fn from_vec(vec: &[u8]) -> Self {
        Self {
            data: vec.to_vec(),
            offset: 0,
            endian: Endian::Little,
            push_offsets: vec![],
            debug: false,
        }
    }

    pub fn from_file(file: &mut File) -> std::io::Result<Self> {
        let mut data = vec![];
        file.read_to_end(&mut data)?;
        Ok(Self {
            data,
            offset: 0,
            endian: Endian::Little,
            push_offsets: vec![],
            debug: false,
        })
    }
}

impl ByteReader {
    pub fn set_debug(&mut self, debug: bool) -> &mut Self {
        self.debug = debug;
        self
    }
    pub fn set_endian(&mut self, endian: Endian) -> &mut Self {
        self.endian = endian;
        self
    }
    pub fn move_to(&mut self, offset: usize) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn jump(&mut self, jump_by: usize) -> &mut Self {
        self.offset += jump_by;
        self
    }

    pub fn push_index(&mut self) -> &mut Self {
        self.push_offsets.push(self.offset);
        self
    }
    pub fn pop_index(&mut self) -> &mut Self {
        if let Some(offset) = self.push_offsets.pop() {
            self.offset = offset;
        }
        self
    }

    pub fn get_file_length(&self) -> usize {
        self.data.len()
    }

    pub fn get_current_offset(&self) -> usize {
        self.offset
    }

    pub fn read_expect(&mut self, cmp_buffer: &[u8]) -> Result<bool, ByteReaderError> {
        let bytes = self.read_bytes(cmp_buffer.len())?;

        for (a, b) in cmp_buffer.iter().zip(bytes.iter()) {
            if a != b {
                return Ok(false);
            }
        }
        return Ok(true);
    }

    pub fn read_bytes(&mut self, bytes: usize) -> Result<&[u8], ByteReaderError> {
        // Was hoping for an easier way by using peak_bytes, but couldn't figure out with out the borrowing problems
        // --- example ---
        // let data = self.peak_bytes()?;
        // self.offset += bytes;
        // return Ok(data)

        if self.offset + bytes > self.data.len() {
            return Err(ByteReaderError::OutOfBounds {
                length: self.data.len(),
                start: self.offset,
                end: self.offset + bytes,
            });
        }

        let data = self.data.get(self.offset..self.offset + bytes).ok_or(
            ByteReaderError::OutOfBounds {
                length: self.data.len(),
                start: self.offset,
                end: self.offset + bytes,
            },
        )?;

        self.offset += bytes;

        if self.debug {
            println!("Read bytes: '{data:?}'  |  Offset: {}", self.offset);
        }

        Ok(data)
    }
    pub fn peak_bytes(&self, bytes: usize) -> Result<&[u8], ByteReaderError> {
        if self.offset + bytes > self.data.len() {
            return Err(ByteReaderError::OutOfBounds {
                length: self.data.len(),
                start: self.offset,
                end: self.offset + bytes,
            });
        }

        let data = self.data.get(self.offset..self.offset + bytes).ok_or(
            ByteReaderError::OutOfBounds {
                length: self.data.len(),
                start: self.offset,
                end: self.offset + bytes,
            },
        )?;
        Ok(data)
    }

    pub fn read_rest(&mut self) -> Result<&[u8], ByteReaderError> {
        self.read_bytes(self.data.len() - self.offset)
    }

    pub fn peak_rest(&self) -> Result<&[u8], ByteReaderError> {
        self.peak_bytes(self.data.len() - self.offset)
    }

    pub fn find_next(&self, sequence: &[u8]) -> Result<usize, ByteReaderError> {
        self.find_from(sequence, self.offset)
    }
    pub fn find(&self, sequence: &[u8]) -> Result<usize, ByteReaderError> {
        self.find_from(sequence, 0)
    }

    pub fn find_all_offsets(&self, sequence: &[u8]) -> Vec<usize> {
        self.find_all_offsets_after(0, sequence)
    }

    pub fn find_all_offsets_after(&self, start_offset: usize, sequence: &[u8]) -> Vec<usize> {
        let mut offset = start_offset;
        let mut found_offsets = vec![];
        while let Ok(found_offset) = self.find_from(sequence, offset) {
            found_offsets.push(found_offset);

            offset = found_offset + 1
        }
        found_offsets
    }

    pub fn find_from(&self, sequence: &[u8], mut offset: usize) -> Result<usize, ByteReaderError> {
        let bytes = sequence.len();

        'data_loop: while let Some(data) = self.data.get(offset..offset + bytes) {
            for (i, byte) in data.iter().enumerate() {
                if sequence[i] != *byte {
                    offset += 1;
                    continue 'data_loop;
                }
            }
            return Ok(offset);
        }

        Err(ByteReaderError::NotFound {
            sequence: sequence.to_vec(),
        })
    }
    pub fn read_string_length<T: FromByteReader + Into<usize>>(
        &mut self,
    ) -> Result<String, ByteReaderError> {
        let length = self.read::<T>()?;
        self.read_string(length.into())
    }
    pub fn read_string_lossy_length<T: FromByteReader + Into<usize>>(
        &mut self,
    ) -> Result<String, ByteReaderError> {
        let length = self.read::<T>()?;
        self.read_string_lossy(length.into())
    }

    pub fn read_string(&mut self, length: usize) -> Result<String, ByteReaderError> {
        String::from_utf8(self.read_bytes(length)?.to_vec()).map_err(ByteReaderError::Utf8Error)
    }

    pub fn read_string_lossy(&mut self, length: usize) -> Result<String, ByteReaderError> {
        Ok(String::from_utf8_lossy(self.read_bytes(length)?).to_string())
    }

    pub fn read<T: FromByteReader>(&mut self) -> Result<T, ByteReaderError> {
        let v = T::read_from_byte_reader(self)?;
        Ok(v)
    }

    pub fn peak<T: FromByteReader>(&mut self) -> Result<T, ByteReaderError> {
        T::peak_from_byte_reader(self)
    }
}

pub trait FromByteReader {
    fn read_from_byte_reader(reader: &mut ByteReader) -> Result<Self, ByteReaderError>
    where
        Self: Sized;

    fn peak_from_byte_reader(reader: &mut ByteReader) -> Result<Self, ByteReaderError>
    where
        Self: Sized,
    {
        reader.push_index();
        let v = Self::read_from_byte_reader(reader)?;
        reader.pop_index();
        Ok(v)
    }
}

macro_rules! impl_from_byte_reader {
    ($ty:ident) => {
        impl FromByteReader for $ty {
            fn read_from_byte_reader(
                reader: &mut crate::ByteReader,
            ) -> Result<$ty, ByteReaderError> {
                let bytes = std::mem::size_of::<Self>();
                let data: [u8; std::mem::size_of::<Self>()] = reader
                    .read_bytes(bytes)?
                    .try_into()
                    .map_err(|_| ByteReaderError::FailedTypeCast {
                        from: format!("&[{}]", stringify!($ty)),
                        to: format!("[{}; {}]", stringify!($ty), bytes),
                    })?;
                match reader.endian {
                    Endian::Little => Ok($ty::from_le_bytes(data)),
                    Endian::Big => Ok($ty::from_be_bytes(data)),
                }
            }
        }
    };
}

impl_from_byte_reader!(u8);
impl_from_byte_reader!(i8);
impl_from_byte_reader!(u16);
impl_from_byte_reader!(i16);
impl_from_byte_reader!(u32);
impl_from_byte_reader!(i32);
impl_from_byte_reader!(u64);
impl_from_byte_reader!(i64);
impl_from_byte_reader!(f32);
impl_from_byte_reader!(f64);
