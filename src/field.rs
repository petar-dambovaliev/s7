// Copyright 2019 Petar Dambovaliev. All rights reserved.
// This software may be modified and distributed under the terms
// of the BSD license. See the LICENSE file for details.

//! Parses bytes from `Area::DataBausteine` to types for easier manipulation

use super::error::Error;
use byteorder::{BigEndian, ByteOrder};

/// Fields collection type alias for convenience
/// # Examples
///
/// ```
/// use s7::field::{Float, Bool, Fields};
///
/// let float = Float::new(888, 8.0, vec![66, 86, 0, 0]).unwrap();
/// let boolean = Bool::new(888, 8.0, vec![1u8]).unwrap();
/// println!("bool: {}", boolean.value());
/// println!("float: {}", float.value());
/// let fields: Fields = vec![Box::new(float), Box::new(boolean)];
///
/// for field in fields.iter() {
///     println!(
///         "saving bytes {:?} to block {} offset {}",
///         field.to_bytes(),
///         field.data_block(),
///         field.offset()
///     )
/// }
/// ```
pub type Fields = Vec<Box<dyn Field>>;

/// represents a type stored in the hardware
/// ie `bool`, `real(32 bit float)`
pub trait Field {
    /// data block
    fn data_block(&self) -> i32;
    /// offset in the data block
    /// for convenience, we truncate the float
    /// we don't care about the digits after the decimal point anymore
    fn offset(&self) -> i32;

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

        let bit_offset = ((offset * 10.0) as usize % 10) as u8;
        if bit_offset != 0 {
            return Err(Error::TryFrom(
                bytes,
                format!(
                    "Float.new: float should not have a bit offset got {}",
                    bit_offset
                ),
            ));
        }

        Ok(Float {
            data_block,
            offset,
            value: BigEndian::read_f32(bytes.as_mut_slice()),
        })
    }

    pub fn size() -> i32 {
        4
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, v: f32) {
        self.value = v
    }
}

impl Field for Float {
    fn data_block(&self) -> i32 {
        self.data_block
    }

    fn offset(&self) -> i32 {
        self.offset as i32
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8; Float::size() as usize];
        BigEndian::write_f32(buf.as_mut_slice(), self.value);
        return buf;
    }
}

#[derive(Debug)]
pub struct Double {
    data_block: i32,
    /// offset example 8.1
    /// left side is index within the block
    /// right side is the bit position only used for bool, zero for all other types
    offset: f64,
    value: f64,
}

impl Double {
    pub fn new(data_block: i32, offset: f64, mut bytes: Vec<u8>) -> Result<Double, Error> {
        let len = bytes.len();
        if bytes.len() != Double::size() as usize {
            return Err(Error::TryFrom(
                bytes,
                format!("Double.new: expected buf size {} got {}", Double::size(), len),
            ));
        }

        let bit_offset = ((offset * 10.0) as usize % 10) as u8;
        if bit_offset != 0 {
            return Err(Error::TryFrom(
                bytes,
                format!(
                    "Double.new: double should not have a bit offset got {}",
                    bit_offset
                ),
            ));
        }

        Ok(Double {
            data_block,
            offset,
            value: BigEndian::read_f64(bytes.as_mut_slice()),
        })
    }

    pub fn size() -> i32 {
        8
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v
    }
}

impl Field for Double {
    fn data_block(&self) -> i32 {
        self.data_block
    }

    fn offset(&self) -> i32 {
        self.offset as i32
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8; Double::size() as usize];
        BigEndian::write_f64(buf.as_mut_slice(), self.value);
        return buf;
    }
}


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
        return b & !(1 << bit_pos);
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
        return buf;
    }
}


#[test]
fn test_fields() {
    let float = Float::new(888, 8.0, vec![66, 86, 0, 0]).unwrap();
    let boolean = Bool::new(888, 8.0, vec![1u8]).unwrap();
    assert!(boolean.value());
    assert_eq!(53.5, float.value());
    let fields: Fields = vec![Box::new(float), Box::new(boolean)];

    for field in fields.iter() {
        println!(
            "saving bytes {:?} to block {} offset {}",
            field.to_bytes(),
            field.data_block(),
            field.offset()
        )
    }
}

#[test]
fn test_float() {
    let val: f32 = 53.5;
    let mut b = vec![0u8; Float::size() as usize];
    BigEndian::write_f32(b.as_mut_slice(), val);
    let mut field = Float::new(888, 8.0, b).unwrap();
    field.set_value(val);
    let result = field.to_bytes();

    assert_eq!(vec![66, 86, 0, 0], result);

    // test invalid bit offset
    // float should not have a bit offset
    match Float::new(888, 8.1, vec![66, 86, 0, 0]) {
        Ok(_) => {
            println!("should return an error at invalid bit offset 1. Floats should not have a bit offset");
            assert!(false)
        }
        Err(_) => {}
    }
}

#[test]
fn test_bool() {
    let b = vec![1u8; 1];
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

    let bb = vec![0b00001000u8; 1];
    field = Bool::new(888, 8.4, bb).unwrap();
    field.set_value(true);

    res = field.to_bytes();

    assert_eq!(res.len(), 1);
    assert_eq!(res[0], 24);
    assert_eq!(field.value(), true);

    // test invalid bit offset
    match Bool::new(888, 8.8, vec![0b00001000u8; 1]) {
        Ok(_) => {
            println!("should return an error at invalid bit offset 8");
            assert!(false)
        }
        Err(_) => {}
    }
}

#[test]
fn test_word() {
    let val: u16 = 43981;
    let mut b = vec![0u8; Word::size() as usize];
    BigEndian::write_u16(b.as_mut_slice(), val);
    let mut field = Word::new(888, 8.0, b).unwrap();
    field.set_value(val);
    let result = field.to_bytes();

    assert_eq!(vec![171, 205], result);

    // test invalid bit offset
    // words should not have a bit offset
    match Word::new(888, 8.1, vec![12, 23]) {
        Ok(_) => {
            println!("should return an error at invalid bit offset 1. Words should not have a bit offset");
            assert!(false)
        }
        Err(_) => {}
    }
}
