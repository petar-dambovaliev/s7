use crate::error::Error;

// Area ID
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(crate) enum Area {
    ProcessInput = 0x81,
    ProcessOutput = 0x82,
    /// Merkers are address registers within the CPU.
    /// The number of available flag bytes depends on the respective CPU and can be taken from the technical data.
    /// You can use flag bits, flag bytes, flag words or flag double words in a PLC program.
    Merker = 0x83,
    /// German thing, means building blocks
    /// This is your storage  
    DataBausteine = 0x84,
    Counter = 0x1C,
    Timer = 0x1D,
    Unknown,
}

// Word Length
pub const WL_BIT: i32 = 0x01; //Bit (inside a word)
pub const WL_BYTE: i32 = 0x02; //Byte (8 bit)
pub const WL_CHAR: i32 = 0x03;
pub const WL_WORD: i32 = 0x04; //Word (16 bit)
pub const WL_INT: i32 = 0x05;
pub const WL_DWORD: i32 = 0x06; //Double Word (32 bit)
pub const WL_DINT: i32 = 0x07;
pub const WL_REAL: i32 = 0x08; //Real (32 bit float)
pub const WL_COUNTER: i32 = 0x1C; //Counter (16 bit)
pub const WL_TIMER: i32 = 0x1D; //Timer (16 bit)

//dataSize to number of byte accordingly
pub fn data_size_byte(word_length: i32) -> i32 {
    match word_length {
        WL_BIT | WL_BYTE | WL_CHAR => 1,
        WL_WORD | WL_INT | WL_COUNTER | WL_TIMER => 2,
        WL_DWORD | WL_DINT | WL_REAL => 4,
        _ => 0,
    }
}

// PLC Status
pub enum CpuStatus {
    Unknown = 0,
    Stop = 4,
    Run = 8,
}

impl CpuStatus {
    pub(crate) fn from_u8(value: u8) -> Result<CpuStatus, Error> {
        match value {
            0 => Ok(CpuStatus::Unknown),
            4 => Ok(CpuStatus::Stop),
            8 => Ok(CpuStatus::Run),
            _ => Err(Error::InvalidCpuStatus(value)),
        }
    }
}

//size header
pub const SIZE_HEADER_READ: i32 = 31; // Header Size when Reading
pub const SIZE_HEADER_WRITE: i32 = 35; // Header Size when Writing

// Result transport size
pub const TS_RES_BIT: i32 = 3;
pub const TS_RES_BYTE: i32 = 4;
#[allow(dead_code)]
pub const TS_RES_INT: i32 = 5;
//todo implement read write multi
#[allow(dead_code)]
pub const TS_RES_REAL: i32 = 7;
pub const TS_RES_OCTET: i32 = 9;
