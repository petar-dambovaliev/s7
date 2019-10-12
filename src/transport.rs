// Copyright 2019 Petar Dambovaliev. All rights reserved.
// This software may be modified and distributed under the terms
// of the BSD license. See the LICENSE file for details.

use super::constant;
use super::error::Error;

/// Client Connection Type
#[derive(Debug, Copy, Clone)]
pub enum Connection {
    /// Connect to the PLC as a PG (Programmiergeräte). German for programming device.
    PG = 1,
    /// Connect to the PLC as an OP (Operator). Console operator control.
    OP = 2,
    /// Basic connection
    Basic = 3,
}

/// an abstract communication used by the client to send requests
/// ## How can I implement `Transport`?
///
/// Types that are [`Transport`] should store the `pdu_length`
/// at the connection phase `self.pdu_length = BigEndian::read_u16(&response[25..]) as i32;`
pub trait Transport {
    /// send request to the plc.
    /// returns a response and an error, if there was any.
    fn send(&mut self, request: &[u8]) -> Result<Vec<u8>, Error>;
    /// pdu length needs to be set by the implementor, during the connection phase.
    fn pdu_length(&self) -> i32;
}

/// response from the plc that the connection has been confirmed
pub const CONFIRM_CONNECTION: u8 = 0xD0;

/// ISO Connection Request telegram (contains also ISO Header and COTP Header)
/// TPKT (RFC1006 Header)
pub const ISO_CONNECTION_REQUEST_TELEGRAM: [u8; 22] = [
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

/// S7 Read/Write Request Header (contains also ISO Header and COTP Header)
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
    0,                       // Data Length = Size(bytes) + 4
    4,                       // Function 4 Read Var, 5 Write Var
    1,                       // Items count
    18,                      // Var spec.
    10,                      // Length of remaining bytes
    16,                      // Syntax ID
    constant::WL_BYTE as u8, // Transport Size idx=22
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

// used during establishing a connection
pub const PDU_NEGOTIATION_TELEGRAM: [u8; 25] = [
    3, 0, 0, 25, 2, 240, 128, // TPKT + COTP (see above for info)
    50, 1, 0, 0, 4, 0, 0, 8, 0, 0, 240, 0, 0, 1, 0, 1, 0, 30,
]; // PDU Length Requested = HI-LO Here Default 480 bytes
