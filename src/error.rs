// Copyright 2019 Petar Dambovaliev. All rights reserved.
// This software may be modified and distributed under the terms
// of the BSD license. See the LICENSE file for details.

use std::error;
use std::fmt;
use std::io::{Error as IOError, ErrorKind};

const TCP_SOCKET_CREATION: i32 = 1;
const TCP_CONNECTION_TIMEOUT: i32 = 2;
const TCP_CONNECTION_FAILED: i32 = 3;
const TCP_RECEIVE_TIMEOUT: i32 = 4;
const TCP_DATA_RECEIVE: i32 = -5;
const TCP_SEND_TIMEOUT: i32 = 0x00000006;
const TCP_DATA_SEND: i32 = 0x00000007;
const TCP_CONNECTION_RESET: i32 = 0x00000008;
const TCP_NOT_CONNECTED: i32 = 0x00000009;
const TCP_UNREACHALE_HOST: i32 = 0x00002751;

const ISO_CONNECT: i32 = 0x00010000;
pub(crate) const ISO_INVALID_PDU: i32 = 0x00030000; // Bad format
pub(crate) const ISO_INVALID_DATA_SIZE: i32 = 0x00040000;

pub(crate) const CLI_NEGOTIATING_PDU: i32 = 0x00100000;
const CLI_INVALID_PARAMS: i32 = 0x00200000;
const CLI_JOB_PENDING: i32 = 0x00300000;
const CLI_TOO_MANY_ITEMS: i32 = 0x00400000;
const CLI_INVALID_DWORD_LEN: i32 = 0x00500000;
const CLI_PARTIAL_DATA_WRITTEN: i32 = 0x00600000;
const CLI_SIZE_OVER_PDU: i32 = 0x00700000;
pub(crate) const CLI_INVALID_PLC_ANSWER: i32 = 0x00800000;
const CLI_ADDRESS_OUT_OF_RANGE: i32 = 0x00900000;
const CLI_INVALID_TRANSPORT_SIZE: i32 = 0x00A00000;
const CLI_WRITE_DATA_SIZE_MISMATCH: i32 = 0x00B00000;
const CLI_ITEM_NOT_AVAILABLE: i32 = 0x00C00000;
const CLI_INVALID_VALUE: i32 = 0x00D00000;
pub(crate) const CLI_CANNOT_START_PLC: i32 = 0x00E00000;
pub(crate) const CLI_ALREADY_RUN: i32 = 0x00F00000;
pub(crate) const CLI_CANNOT_STOP_PLC: i32 = 0x01000000;
const CLI_CANNOT_COPY_RAM_TO_ROM: i32 = 0x01100000;
const CLI_CANNOT_COMPRESS: i32 = 0x01200000;
pub(crate) const CLI_ALREADY_STOP: i32 = 0x01300000;
const CLI_FUN_NOT_AVAILABLE: i32 = 0x01400000;
const CLI_UPLOAD_SEQUENCE_FAILED: i32 = 0x01500000;
const CLI_INVALID_DATA_SIZE_RECVD: i32 = 0x01600000;
const CLI_INVALID_BLOCK_TYPE: i32 = 0x01700000;
const CLI_INVALID_BLOCK_NUMBER: i32 = 0x01800000;
const CLI_INVALID_BLOCK_SIZE: i32 = 0x01900000;
const CLI_NEED_PASSWORD: i32 = 0x01D00000;
const CLI_INVALID_PASSWORD: i32 = 0x01E00000;
const CLI_NO_PASSWORD_TO_SET_OR_CLEAR: i32 = 0x01F00000;
const CLI_JOB_TIMEOUT: i32 = 0x02000000;
const CLI_PARTIAL_DATA_READ: i32 = 0x02100000;
const CLI_BUFFER_TOO_SMALL: i32 = 0x02200000;
const CLI_FUNCTION_REFUSED: i32 = 0x02300000;
const CLI_DESTROYING: i32 = 0x02400000;
const CLI_INVALID_PARAM_NUMBER: i32 = 0x02500000;
const CLI_CANNOT_CHANGE_PARAM: i32 = 0x02600000;
const CLI_FUNCTION_NOT_IMPLEMENTED: i32 = 0x02700000;

const CODE_7_ADDRESS_OUT_OF_RANGE: i32 = 5;
const CODE_7_INVALID_TRANSPORT_SIZE: i32 = 6;
const CODE_7_WRITE_DATA_SIZE_MISMATCH: i32 = 7;
const CODE_7_RES_ITEM_NOT_AVAILABLE: i32 = 10;
const CODE_7_RES_ITEM_NOT_AVAILABLE1: i32 = 53769;
const CODE_7_INVALID_VALUE: i32 = 56321;
const CODE_7_NEED_PASSWORD: i32 = 53825;
const CODE_7_INVALID_PASSWORD: i32 = 54786;
const CODE_7_NO_PASSWORD_TO_CLEAR: i32 = 54788;
const CODE_7_NO_PASSWORD_TO_SET: i32 = 54789;
const CODE_7_FUN_NOT_AVAILABLE: i32 = 33028;
const CODE_7_DATA_OVER_PDU: i32 = 34048;

#[derive(Debug)]
pub enum Error {
    Connect(String),
    Lock,
    IOError(ErrorKind),
    Response { code: i32 },
    CPU { code: i32 },
    InvalidInput { input: String },
    Send,
    Iso,
    PduLength(i32),
    TryFrom(Vec<u8>, String),
    InvalidCpuStatus(u8),
    InvalidResponse { reason: String, bytes: Vec<u8> },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Connect(s) => write!(f, "connection error: {}", s),
            Error::Lock => write!(f, "Lock error: panicked"),
            Error::IOError(kind) => write!(f, "IO error: {:?}", kind),
            Error::Response { code } => write!(f, "Error response: {}", error_text(*code)),
            Error::CPU { code } => {
                write!(f, "Error response CPU: {}", error_text(cpu_error(*code)))
            }
            Error::InvalidInput { input } => write!(f, "Invalid input: {}", input),
            Error::Send => write!(f, "Send connection error"),
            Error::Iso => write!(f, "ISO connection error"),
            Error::PduLength(pdu) => write!(f, "PDU length connection error {}", pdu),
            Error::TryFrom(bytes, reason) => {
                write!(f, "Could not read bytes {:?} reason {}", bytes, reason)
            }
            Error::InvalidCpuStatus(status) => write!(f, "Invalid cpu status {}", status),
            Error::InvalidResponse { reason, bytes } => {
                write!(f, "Invalid response {:?} err {}", bytes, reason)
            }
        }
    }
}

impl From<IOError> for Error {
    fn from(e: IOError) -> Self {
        Error::IOError(e.kind())
    }
}
// This is important for other errors to wrap this one.
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

//CPUError specific CPU error after response
fn cpu_error(err: i32) -> i32 {
    match err {
        CODE_7_ADDRESS_OUT_OF_RANGE => CLI_ADDRESS_OUT_OF_RANGE,
        CODE_7_INVALID_TRANSPORT_SIZE => CLI_INVALID_TRANSPORT_SIZE,
        CODE_7_WRITE_DATA_SIZE_MISMATCH => CLI_WRITE_DATA_SIZE_MISMATCH,
        CODE_7_RES_ITEM_NOT_AVAILABLE | CODE_7_RES_ITEM_NOT_AVAILABLE1 => CLI_ITEM_NOT_AVAILABLE,
        CODE_7_DATA_OVER_PDU => CLI_SIZE_OVER_PDU,
        CODE_7_INVALID_VALUE => CLI_INVALID_VALUE,
        CODE_7_FUN_NOT_AVAILABLE => CLI_FUN_NOT_AVAILABLE,
        CODE_7_NEED_PASSWORD => CLI_NEED_PASSWORD,
        CODE_7_INVALID_PASSWORD => CLI_INVALID_PASSWORD,
        CODE_7_NO_PASSWORD_TO_SET | CODE_7_NO_PASSWORD_TO_CLEAR => CLI_NO_PASSWORD_TO_SET_OR_CLEAR,
        _ => CLI_FUNCTION_REFUSED,
    }
}

//ErrorText return a string error text from error code integer
fn error_text(err: i32) -> &'static str {
    match err {
        0 => "OK",
        TCP_SOCKET_CREATION => "SYS : Error creating the Socket",
        TCP_CONNECTION_TIMEOUT => "TCP : Connection Timeout",
        TCP_CONNECTION_FAILED => "TCP : Connection Error",
        TCP_RECEIVE_TIMEOUT => "TCP : Data receive Timeout",
        TCP_DATA_RECEIVE => "TCP : Error receiving Data",
        TCP_SEND_TIMEOUT => "TCP : Data send Timeout",
        TCP_DATA_SEND => "TCP : Error sending Data",
        TCP_CONNECTION_RESET => "TCP : Connection reset by the Peer",
        TCP_NOT_CONNECTED => "CLI : Client not connected",
        TCP_UNREACHALE_HOST => "TCP : Unreachable host",

        ISO_CONNECT => "ISO : Connection Error",
        ISO_INVALID_PDU => "ISO : Invalid PDU received",
        ISO_INVALID_DATA_SIZE => "ISO : Invalid Buffer passed to Send/Receive",

        CLI_NEGOTIATING_PDU => "CLI : Error in PDU negotiation",
        CLI_INVALID_PARAMS => "CLI : invalid param(s) supplied",
        CLI_JOB_PENDING => "CLI : Job pending",
        CLI_TOO_MANY_ITEMS => "CLI : too may items (>20) in multi read/write",
        CLI_INVALID_DWORD_LEN => "CLI : invalid WordLength",
        CLI_PARTIAL_DATA_WRITTEN => "CLI : Partial data written",
        CLI_SIZE_OVER_PDU => "CPU : total data exceeds the PDU size",
        CLI_INVALID_PLC_ANSWER => "CLI : invalid CPU answer",
        CLI_ADDRESS_OUT_OF_RANGE => "CPU : Address out of range",
        CLI_INVALID_TRANSPORT_SIZE => "CPU : Invalid Transport size",
        CLI_WRITE_DATA_SIZE_MISMATCH => "CPU : Data size mismatch",
        CLI_ITEM_NOT_AVAILABLE => "CPU : Item not available",
        CLI_INVALID_VALUE => "CPU : Invalid value supplied",
        CLI_CANNOT_START_PLC => "CPU : Cannot start PLC",
        CLI_ALREADY_RUN => "CPU : PLC already RUN",
        CLI_CANNOT_STOP_PLC => "CPU : Cannot stop PLC",
        CLI_CANNOT_COPY_RAM_TO_ROM => "CPU : Cannot copy RAM to ROM",
        CLI_CANNOT_COMPRESS => "CPU : Cannot compress",
        CLI_ALREADY_STOP => "CPU : PLC already STOP",
        CLI_FUN_NOT_AVAILABLE => "CPU : Function not available",
        CLI_UPLOAD_SEQUENCE_FAILED => "CPU : Upload sequence failed",
        CLI_INVALID_DATA_SIZE_RECVD => "CLI : Invalid data size received",
        CLI_INVALID_BLOCK_TYPE => "CLI : Invalid block type",
        CLI_INVALID_BLOCK_NUMBER => "CLI : Invalid block number",
        CLI_INVALID_BLOCK_SIZE => "CLI : Invalid block size",
        CLI_NEED_PASSWORD => "CPU : Function not authorized for current protection level",
        CLI_INVALID_PASSWORD => "CPU : Invalid password",
        CLI_NO_PASSWORD_TO_SET_OR_CLEAR => "CPU : No password to set or clear",
        CLI_JOB_TIMEOUT => "CLI : Job Timeout",
        CLI_FUNCTION_REFUSED => "CLI : function refused by CPU (Unknown error)",
        CLI_PARTIAL_DATA_READ => "CLI : Partial data read",
        CLI_BUFFER_TOO_SMALL => {
            "CLI : The buffer supplied is too small to accomplish the operation"
        }
        CLI_DESTROYING => "CLI : Cannot perform (destroying)",
        CLI_INVALID_PARAM_NUMBER => "CLI : Invalid Param Number",
        CLI_CANNOT_CHANGE_PARAM => "CLI : Cannot change this param now",
        CLI_FUNCTION_NOT_IMPLEMENTED => "CLI : Function not implemented",
        _ => "CLI : Unknown error",
    }
}
