// Copyright 2019 Petar Dambovaliev. All rights reserved.
// This software may be modified and distributed under the terms
// of the BSD license. See the LICENSE file for details.

//! Parses bytes from `Area::DataBausteine` to types for easier manipulation

mod bool;
mod double;
mod float;
mod word;
//todo add string

pub use bool::*;
pub use double::*;
pub use float::*;
pub use word::*;

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

#[cfg(test)]
mod tests {
    use super::*;

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
        Float::new(888, 8.1, vec![66, 86, 0, 0]).expect_err(
            "should return an error at invalid bit offset 1. Floats should not have a bit offset",
        );
    }

    #[test]
    fn test_bool() {
        let b = vec![1u8; 1];
        let mut field = Bool::new(888, 8.1, b).unwrap();
        field.set_value(true);

        let mut res: Vec<u8> = field.to_bytes();

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], 3);
        assert!(field.value());

        field.set_value(false);
        res = field.to_bytes();

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], 1);
        assert!(!field.value());

        let bb = vec![0b00001000u8; 1];
        field = Bool::new(888, 8.4, bb).unwrap();
        field.set_value(true);

        res = field.to_bytes();

        assert_eq!(res.len(), 1);
        assert_eq!(res[0], 24);
        assert!(field.value());

        // test invalid bit offset
        Bool::new(888, 8.8, vec![0b00001000u8; 1])
            .expect_err("should return an error at invalid bit offset 8");
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
        Word::new(888, 8.1, vec![12, 23]).expect_err(
            "should return an error at invalid bit offset 1. Words should not have a bit offset",
        );
    }
}
