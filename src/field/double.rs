use super::*;

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
                format!(
                    "Double.new: expected buf size {} got {}",
                    Double::size(),
                    len
                ),
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
        buf
    }
}
