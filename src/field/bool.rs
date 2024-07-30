use super::*;

/// Bool represents a single bit in a byte from `Area::DataBausteine`
#[derive(Debug)]
pub struct Bool {
    /// index of the block it's stored at
    data_block: i32,
    /// example 8.1
    /// left side is index within the block
    /// right side is the bit position only used for bool, zero for all other types
    offset: f32,
    /// the actual primitive value
    byte: u8,
    /// the current value that will be written to the byte
    value: bool,
}

impl Bool {
    pub fn new(data_block: i32, offset: f32, bytes: Vec<u8>) -> Result<Self, Error> {
        let len = bytes.len();
        if bytes.len() != Self::size() as usize {
            return Err(Error::TryFrom(
                bytes,
                format!("Bool.new: expected buf size {} got {}", Self::size(), len),
            ));
        }

        let bit_offset = ((offset * 10.0) as usize % 10) as u8;
        if bit_offset > 7 {
            return Err(Error::TryFrom(
                bytes,
                format!("Bool.new: max offset is 7 got {}", offset),
            ));
        }

        Ok(Bool {
            data_block,
            offset,
            byte: bytes[0],
            value: bytes[0] & (1 << bit_offset) != 0,
        })
    }

    #[inline(always)]
    fn set_value_at(b: u8, bit_pos: u8, val: bool) -> u8 {
        if val {
            return b | (1 << bit_pos);
        }
        b & !(1 << bit_pos)
    }

    pub fn size() -> i32 {
        1
    }

    /// gets the value at the current offset
    pub fn value(&self) -> bool {
        self.value
    }

    pub fn set_value(&mut self, v: bool) {
        self.value = v;
        self.byte = Bool::set_value_at(
            self.byte,
            ((self.offset * 10.0) as usize % 10) as u8,
            self.value,
        );
    }
}

impl Field for Bool {
    fn data_block(&self) -> i32 {
        self.data_block
    }

    fn offset(&self) -> i32 {
        self.offset as i32
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.byte]
    }
}
