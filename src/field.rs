// Copyright 2019 Petar Dambovaliev. All rights reserved.
// This software may be modified and distributed under the terms
// of the BSD license. See the LICENSE file for details.

//! Parses bytes from `Area::DataBausteine` to types for easier manipulation

use super::error::Error;
use byteorder::{BigEndian, ByteOrder};

/// represents a type stored in the hardware
/// ie `bool`, `real(32 bit float)`
pub trait Field<T, V> {
    /// data block
    fn data_block(&self) -> i32;
    /// offset in the data block
    /// for convenience, we truncate the float
    /// we don't care about the digits after the decimal point anymore
    fn offset(&self) -> i32;
    /// the field byte count
    /// `bools` are 1 bit within a byte
    fn size() -> i32;
    /// the actual primitive type
    fn value(&self) -> V;

    fn set_value(&mut self, v: V);

    fn to_bytes(&self) -> Vec<u8>;
}

/// PLC float field
#[derive(Debug)]
pub struct Float {
    data_block: i32,
    /// offset example 8.1
    /// left side is index within the block
    /// right side is the bit position only used for bool, zero for all other types
    offset: f32,
    value: f32,
}

impl Float {
    pub fn new(data_block: i32, offset: f32, mut bytes: Vec<u8>) -> Result<Float, Error> {
        let len = bytes.len();
        if bytes.len() != Float::size() as usize {
            return Err(Error::TryFrom(
                bytes,
                format!("Float.new: expected buf size {} got {}", Float::size(), len),
            ));
        }
        Ok(Float {
            data_block,
            offset,
            value: BigEndian::read_f32(bytes.as_mut_slice()),
        })
    }
}

impl Field<Vec<u8>, f32> for Float {
    fn data_block(&self) -> i32 {
        self.data_block
    }

    fn offset(&self) -> i32 {
        self.offset as i32
    }

    fn size() -> i32 {
        4
    }

    fn value(&self) -> f32 {
        self.value
    }

    fn set_value(&mut self, v: f32) {
        self.value = v
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8; Float::size() as usize];
        BigEndian::write_f32(buf.as_mut_slice(), self.value);
        return buf;
    }
}

/// BoolBlock represents a single bit in a byte from `Area::DataBausteine`
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
                format!("Float.new: expected buf size {} got {}", Self::size(), len),
            ));
        }
        Ok(Bool {
            data_block,
            offset,
            byte: bytes[0],
            value: bytes[0] & (1 << ((offset * 10.0) as usize % 10) as u8) != 0,
        })
    }

    #[inline(always)]
    fn set_value_at(b: u8, bit_pos: u8, val: bool) -> u8 {
        if val {
            return b | (1 << bit_pos);
        }
        return b & !(1 << bit_pos);
    }
}

impl Field<Vec<u8>, bool> for Bool {
    fn data_block(&self) -> i32 {
        self.data_block
    }

    fn offset(&self) -> i32 {
        self.offset as i32
    }

    fn size() -> i32 {
        1
    }

    /// gets the value at the current offset
    fn value(&self) -> bool {
        self.value
    }

    fn set_value(&mut self, v: bool) {
        self.value = v;
        self.byte = Bool::set_value_at(
            self.byte,
            ((self.offset * 10.0) as usize % 10) as u8,
            self.value,
        );
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.byte]
    }
}

#[test]
fn test_bool() {
    let mut b = vec![1u8; 1];
    let mut field = Bool::new(888, 8.1, b).unwrap();
    field.set_value(true);

    let mut res: Vec<u8> = field.to_bytes();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0], 3);
    assert_eq!(field.value(), true);

    field.set_value(false);
    res = field.to_bytes();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0], 1);
    assert_eq!(field.value(), false);

    let mut bb = vec![0b00001000u8; 1];
    field = Bool::new(888, 8.4, bb).unwrap();
    field.set_value(true);

    res = field.to_bytes();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0], 24);
    assert_eq!(field.value(), true);
}
