use super::*;

/// PLC word field
#[derive(Debug)]
pub struct Word {
    data_block: i32,
    /// offset example 8.1
    /// left side is index within the block
    /// right side is the bit position only used for bool, zero for all other types
    offset: f32,
    value: u16,
}

impl Word {
    pub fn new(data_block: i32, offset: f32, mut bytes: Vec<u8>) -> Result<Word, Error> {
        let len = bytes.len();
        if bytes.len() != Word::size() as usize {
            return Err(Error::TryFrom(
                bytes,
                format!("Word.new: expected buf size {} got {}", Word::size(), len),
            ));
        }

        let bit_offset = ((offset * 10.0) as usize % 10) as u8;
        if bit_offset != 0 {
            return Err(Error::TryFrom(
                bytes,
                format!(
                    "Word.new: float should not have a bit offset got {}",
                    bit_offset
                ),
            ));
        }

        Ok(Word {
            data_block,
            offset,
            value: BigEndian::read_u16(bytes.as_mut_slice()),
        })
    }

    pub fn size() -> i32 {
        2
    }

    pub fn value(&self) -> u16 {
        self.value
    }

    pub fn set_value(&mut self, v: u16) {
        self.value = v
    }
}

impl Field for Word {
    fn data_block(&self) -> i32 {
        self.data_block
    }

    fn offset(&self) -> i32 {
        self.offset as i32
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8; Word::size() as usize];
        BigEndian::write_u16(buf.as_mut_slice(), self.value);
        buf
    }
}
