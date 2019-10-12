pub const CONFIRM_CONNECTION: u8 = 0xD0;

// ISO Connection Request telegram (contains also ISO Header and COTP Header)
pub const ISO_CONNECTION_REQUEST_TELEGRAM: [u8; 22] = [
    // TPKT (RFC1006 Header)
    3,  // RFC 1006 ID (3)
    0,  // Reserved, always 0
    0,  // High part of packet lenght (entire frame, payload and TPDU included)
    22, // Low part of packet lenght (entire frame, payload and TPDU included)
    // COTP (ISO 8073 Header)
    17,  // PDU Size Length
    224, // CR - Connection Request ID
    0,   // Dst Reference HI
    0,   // Dst Reference LO
    0,   // Src Reference HI
    1,   // Src Reference LO
    0,   // Class + Options Flags
    192, // PDU Max Length ID
    1,   // PDU Max Length HI
    10,  // PDU Max Length LO
    193, // Src TSAP Identifier
    2,   // Src TSAP Length (2 bytes)
    1,   // Src TSAP HI (will be overwritten)
    0,   // Src TSAP LO (will be overwritten)
    194, // Dst TSAP Identifier
    2,   // Dst TSAP Length (2 bytes)
    1,   // Dst TSAP HI (will be overwritten)
    2,
]; // Dst TSAP LO (will be overwritten)

// Area ID
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Area {
    ProcessInput = 0x81,
    ProcessOutput = 0x82,
    Merker = 0x83,
    DataBausteine = 0x84, //German thing, means building blocks
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
pub const CPU_STATUS_UNKNOWN: i32 = 0;
pub const CPU_STATUS_RUN: i32 = 8;
pub const CPU_STATUS_STOP: i32 = 4;

//size header
pub const SIZE_HEADER_READ: i32 = 31; // Header Size when Reading
pub const SIZE_HEADER_WRITE: i32 = 35; // Header Size when Writing

// Result transport size
pub const TS_RES_BIT: i32 = 3;
pub const TS_RES_BYTE: i32 = 4;
pub const TS_RES_INT: i32 = 5;
pub const TS_RES_REAL: i32 = 7;
pub const TS_RES_OCTET: i32 = 9;

// S7 Read/Write Request Header (contains also ISO Header and COTP Header)
pub const READ_WRITE_TELEGRAM: [u8; 35] = [
    // 31-35 bytes
    3,
    0,
    0,
    31, // Telegram Length (Data Size + 31 or 35)
    2,
    240,
    128, // COTP (see above for info)
    50,  // S7 Protocol ID
    1,   // Job Type
    0,
    0, // Redundancy identification
    5,
    0, // PDU Reference //lth this use for request S7 packet id
    0,
    14, // Parameters Length
    0,
    0,             // Data Length = Size(bytes) + 4
    4,             // Function 4 Read Var, 5 Write Var
    1,             // Items count
    18,            // Var spec.
    10,            // Length of remaining bytes
    16,            // Syntax ID
    WL_BYTE as u8, // Transport Size idx=22
    0,
    0, // Num Elements
    0,
    0,   // DB Number (if any, else 0)
    132, // Area Type
    0,
    0,
    0, // Area Offset
    // WR area
    0, // Reserved
    4, // Transport size
    0,
    0,
]; // Data Length * 8 (if not bit or timer or counter)

pub const PDU_NEGOTIATION_TELEGRAM: [u8; 25] = [
    3, 0, 0, 25, 2, 240, 128, // TPKT + COTP (see above for info)
    50, 1, 0, 0, 4, 0, 0, 8, 0, 0, 240, 0, 0, 1, 0, 1, 0, 30,
]; // PDU Length Requested = HI-LO Here Default 480 bytes
