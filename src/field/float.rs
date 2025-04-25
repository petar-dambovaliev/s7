use super::*;

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
        buf
    }
}
