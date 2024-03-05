// Copyright 2019 Petar Dambovaliev. All rights reserved.
// This software may be modified and distributed under the terms
// of the BSD license. See the LICENSE file for details.

use super::constant::{self, Area};
use super::error::{self, Error};
use super::transport::{self, Transport};
use crate::constant::CpuStatus;
use byteorder::{BigEndian, ByteOrder};
use std::str;

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub module_type_name: String,
    pub serial_number: String,
    pub as_name: String,
    pub copyright: String,
    pub module_name: String,
}

#[derive(Debug, Clone)]
pub struct CPInfo {
    pub max_pdu_length: u16,
    pub max_connections: u16,
    pub max_mpi_rate: u16,
    pub max_bus_rate: u16,
}

#[derive(Debug, Clone)]
pub struct Client<T: Transport> {
    transport: T,
}

impl<T: Transport> Client<T> {
    pub fn new(mut transport: T) -> Result<Client<T>, Error> {
        transport.negotiate()?;
        Ok(Client { transport })
    }

    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, IpAddr};
    /// use s7::{client, tcp, transport};
    /// use std::time::Duration;
    /// use s7::field::{Bool, Field};
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    /// let mut opts = tcp::Options::new(IpAddr::from(addr), 5, 5, transport::Connection::PG);
    ///
    /// opts.read_timeout = Duration::from_secs(2);
    /// opts.write_timeout = Duration::from_secs(2);
    ///
    /// let t = tcp::Transport::connect(opts).unwrap();
    /// let mut cl = client::Client::new(t).unwrap();
    ///
    /// let buffer = &mut vec![0u8; Bool::size() as usize];
    /// let db = 888;
    /// let offset = 8.4;
    ///
    /// cl.ag_read(db, offset as i32, Bool::size(), buffer).unwrap();
    ///
    /// let mut  lights = Bool::new(db, offset, buffer.to_vec()).unwrap();
    /// lights.set_value(!lights.value()); // toggle the light switch
    ///
    /// // save
    /// cl.ag_write(
    ///     lights.data_block(),
    ///     lights.offset(),
    ///     Bool::size(),
    ///     lights.to_bytes().as_mut()
    /// ).unwrap();
    ///
    /// ```
    pub fn ag_read(
        &mut self,
        db_number: i32,
        start: i32,
        size: i32,
        buffer: &mut Vec<u8>,
    ) -> Result<(), Error> {
        return self.read(
            Area::DataBausteine,
            db_number,
            start,
            size,
            constant::WL_BYTE,
            buffer,
        );
    }

    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, IpAddr};
    /// use s7::{client, tcp, transport};
    /// use std::time::Duration;
    /// use s7::field::{Bool, Field};
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    /// let mut opts = tcp::Options::new(IpAddr::from(addr), 5, 5, transport::Connection::PG);
    ///
    /// opts.read_timeout = Duration::from_secs(2);
    /// opts.write_timeout = Duration::from_secs(2);
    ///
    /// let t = tcp::Transport::connect(opts).unwrap();
    /// let mut cl = client::Client::new(t).unwrap();
    ///
    /// let buffer = &mut vec![0u8; Bool::size() as usize];
    /// let db = 888;
    /// let offset = 8.4;
    ///
    /// cl.ag_read(db, offset as i32, Bool::size(), buffer).unwrap();
    ///
    /// let mut  lights = Bool::new(db, offset, buffer.to_vec()).unwrap();
    /// lights.set_value(!lights.value()); // toggle the light switch
    ///
    /// // save
    /// cl.ag_write(
    ///     lights.data_block(),
    ///     lights.offset(),
    ///     Bool::size(),
    ///     lights.to_bytes().as_mut()
    /// ).unwrap();
    ///
    /// ```
    pub fn ag_write(
        &mut self,
        db_number: i32,
        start: i32,
        size: i32,
        buffer: &mut Vec<u8>,
    ) -> Result<(), Error> {
        return self.write(
            Area::DataBausteine,
            db_number,
            start,
            size,
            constant::WL_BYTE,
            buffer,
        );
    }

    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, IpAddr};
    /// use s7::{client, tcp, transport};
    /// use std::time::Duration;
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    /// let mut opts = tcp::Options::new(IpAddr::from(addr), 5, 5, transport::Connection::PG);
    ///
    /// opts.read_timeout = Duration::from_secs(2);
    /// opts.write_timeout = Duration::from_secs(2);
    ///
    ///
    /// let t = tcp::Transport::connect(opts).unwrap();
    /// let mut cl = client::Client::new(t).unwrap();
    ///
    /// let buffer = &mut vec![0u8; 255];
    ///
    /// cl.mb_read(1, 3, buffer).unwrap();
    /// ```
    pub fn mb_read(&mut self, start: i32, size: i32, buffer: &mut Vec<u8>) -> Result<(), Error> {
        return self.read(Area::Merker, 0, start, size, constant::WL_BYTE, buffer);
    }

    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, IpAddr};
    /// use s7::{client, tcp, transport};
    /// use std::time::Duration;
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    /// let mut opts = tcp::Options::new(IpAddr::from(addr), 5, 5, transport::Connection::PG);
    ///
    /// opts.read_timeout = Duration::from_secs(2);
    /// opts.write_timeout = Duration::from_secs(2);
    ///
    ///
    /// let t = tcp::Transport::connect(opts).unwrap();
    /// let mut cl = client::Client::new(t).unwrap();
    ///
    /// let buffer = &mut vec![0u8; 255];
    ///
    /// cl.mb_write(1, 3, buffer).unwrap();
    /// ```
    pub fn mb_write(&mut self, start: i32, size: i32, buffer: &mut Vec<u8>) -> Result<(), Error> {
        return self.write(Area::Merker, 0, start, size, constant::WL_BYTE, buffer);
    }

    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, IpAddr};
    /// use s7::{client, tcp, transport};
    /// use std::time::Duration;
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    /// let mut opts = tcp::Options::new(IpAddr::from(addr), 5, 5, transport::Connection::PG);
    ///
    /// opts.read_timeout = Duration::from_secs(2);
    /// opts.write_timeout = Duration::from_secs(2);
    ///
    ///
    /// let t = tcp::Transport::connect(opts).unwrap();
    /// let mut cl = client::Client::new(t).unwrap();
    ///
    /// let buffer = &mut vec![0u8; 255];
    ///
    /// cl.eb_read(1, 3, buffer).unwrap();
    /// ```
    pub fn eb_read(&mut self, start: i32, size: i32, buffer: &mut Vec<u8>) -> Result<(), Error> {
        return self.read(
            Area::ProcessInput,
            0,
            start,
            size,
            constant::WL_BYTE,
            buffer,
        );
    }

    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, IpAddr};
    /// use s7::{client, tcp, transport};
    /// use std::time::Duration;
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    /// let mut opts = tcp::Options::new(IpAddr::from(addr), 5, 5, transport::Connection::PG);
    ///
    /// opts.read_timeout = Duration::from_secs(2);
    /// opts.write_timeout = Duration::from_secs(2);
    ///
    ///
    /// let t = tcp::Transport::connect(opts).unwrap();
    /// let mut cl = client::Client::new(t).unwrap();
    ///
    /// let buffer = &mut vec![0u8; 255];
    ///
    /// cl.eb_write(1, 3, buffer).unwrap();
    /// ```
    pub fn eb_write(&mut self, start: i32, size: i32, buffer: &mut Vec<u8>) -> Result<(), Error> {
        return self.write(
            Area::ProcessInput,
            0,
            start,
            size,
            constant::WL_BYTE,
            buffer,
        );
    }

    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, IpAddr};
    /// use s7::{client, tcp, transport};
    /// use std::time::Duration;
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    /// let mut opts = tcp::Options::new(IpAddr::from(addr), 5, 5, transport::Connection::PG);
    ///
    /// opts.read_timeout = Duration::from_secs(2);
    /// opts.write_timeout = Duration::from_secs(2);
    ///
    ///
    /// let t = tcp::Transport::connect(opts).unwrap();
    /// let mut cl = client::Client::new(t).unwrap();
    ///
    /// let buffer = &mut vec![0u8; 255];
    ///
    /// cl.ab_read(1, 3, buffer).unwrap();
    /// ```
    pub fn ab_read(&mut self, start: i32, size: i32, buffer: &mut Vec<u8>) -> Result<(), Error> {
        return self.read(
            Area::ProcessOutput,
            0,
            start,
            size,
            constant::WL_BYTE,
            buffer,
        );
    }

    /// # Examples
    ///
    /// ```no_run
    /// use std::net::{Ipv4Addr, IpAddr};
    /// use s7::{client, tcp, transport};
    /// use std::time::Duration;
    ///
    /// let addr = Ipv4Addr::new(127, 0, 0, 1);
    /// let mut opts = tcp::Options::new(IpAddr::from(addr), 5, 5, transport::Connection::PG);
    ///
    /// opts.read_timeout = Duration::from_secs(2);
    /// opts.write_timeout = Duration::from_secs(2);
    ///
    ///
    /// let t = tcp::Transport::connect(opts).unwrap();
    /// let mut cl = client::Client::new(t).unwrap();
    ///
    /// let buffer = &mut vec![0u8; 255];
    ///
    /// cl.ab_write(1, 3, buffer).unwrap();
    /// ```
    pub fn ab_write(&mut self, start: i32, size: i32, buffer: &mut Vec<u8>) -> Result<(), Error> {
        return self.write(
            Area::ProcessOutput,
            0,
            start,
            size,
            constant::WL_BYTE,
            buffer,
        );
    }

    //read generic area, pass result into a buffer
    fn read(
        &mut self,
        area: Area,
        db_number: i32,
        mut start: i32,
        mut amount: i32,
        mut word_len: i32,
        buffer: &mut Vec<u8>,
    ) -> Result<(), Error> {
        // Some adjustment
        match area {
            Area::Counter => word_len = constant::WL_COUNTER,
            Area::Timer => word_len = constant::WL_TIMER,
            _ => {}
        };

        // Calc Word size
        let mut word_size = constant::data_size_byte(word_len);

        if word_size == 0 {
            return Err(Error::Response {
                code: error::ISO_INVALID_DATA_SIZE,
            });
        }

        if word_len == constant::WL_BIT {
            amount = 1; // Only 1 bit can be transferred at time
        } else {
            if word_len != constant::WL_COUNTER && word_len != constant::WL_TIMER {
                amount = amount * word_size;
                word_size = 1;
                word_len = constant::WL_BYTE;
            }
        }

        let pdu_length = self.transport.pdu_length();

        if pdu_length == 0 {
            return Err(Error::PduLength(pdu_length));
        }

        let max_elements = (pdu_length - 18) / word_size; // 18 = Reply telegram header //lth note here

        let mut tot_elements = amount;
        let db_bytes = (db_number as u16).to_be_bytes();
        let mut offset = 0;

        while tot_elements > 0 {
            let mut num_elements = tot_elements;

            if num_elements > max_elements {
                num_elements = max_elements;
            }

            let size_requested = num_elements * word_size;
            // Setup the telegram
            let mut request =
                transport::READ_WRITE_TELEGRAM[..constant::SIZE_HEADER_READ as usize].to_vec();

            // Set DB Number
            request[25] = db_bytes[0];
            request[26] = db_bytes[1];

            // Set Area
            request[27] = area as u8;
            // match area {
            //     Area::DataBausteine => request[27] = area as u8,
            //     _ => {}
            // }

            // Adjusts Start and word length
            let mut address = match word_len {
                constant::WL_BIT | constant::WL_COUNTER | constant::WL_TIMER => {
                    request[22] = word_len as u8;
                    start
                }
                _ => start << 3,
            };

            // Num elements
            let num_elements_bytes = (num_elements as u16).to_be_bytes();
            request[23] = num_elements_bytes[0];
            request[24] = num_elements_bytes[1];

            // Address into the PLC (only 3 bytes)
            request[30] = (address & 0x0FF) as u8;
            address = address >> 8;
            request[29] = (address & 0x0FF) as u8;
            address = address >> 8;
            request[28] = (address & 0x0FF) as u8;

            let result = self.transport.send(request.as_slice());

            match result {
                Ok(response) => {
                    if response.len() < 25 {
                        return Err(Error::Response {
                            code: error::ISO_INVALID_DATA_SIZE,
                        });
                    }

                    if response[21] != 0xFF {
                        return Err(Error::CPU {
                            code: response[21] as i32,
                        });
                    }
                    let (mut i, end): (usize, usize) = (25, 25 + (size_requested as usize));

                    //copy response to buffer
                    for k in offset..size_requested {
                        if i == end {
                            break;
                        }
                        buffer[k as usize] = response[i];
                        i += 1;
                    }
                    offset += size_requested;
                }
                Err(e) => {
                    return Err(e);
                }
            }

            tot_elements -= num_elements;
            start += num_elements * word_size
        }
        Ok(())
    }

    fn write(
        &mut self,
        area: Area,
        db_number: i32,
        mut start: i32,
        mut amount: i32,
        mut word_len: i32,
        buffer: &mut Vec<u8>,
    ) -> Result<(), Error> {
        // Some adjustment
        word_len = match area {
            Area::Counter => constant::WL_COUNTER,
            Area::Timer => constant::WL_TIMER,
            _ => word_len,
        };

        // Calc Word size
        let mut word_size = constant::data_size_byte(word_len);

        if word_size == 0 {
            return Err(Error::Response {
                code: error::ISO_INVALID_DATA_SIZE,
            });
        }

        if word_len == constant::WL_BIT {
            amount = 1; // Only 1 bit can be transferred at time
        } else {
            if word_len != constant::WL_COUNTER && word_len != constant::WL_TIMER {
                amount = amount * word_size;
                word_size = 1;
                word_len = constant::WL_BYTE;
            }
        }

        let mut offset: i32 = 0;
        let pdu_length = self.transport.pdu_length();
        let max_elements = (pdu_length - 35) / word_size; // 35 = Reply telegram header
        let mut tot_elements = amount;

        while tot_elements > 0 {
            let mut num_elements = tot_elements;
            if num_elements > max_elements {
                num_elements = max_elements;
            }
            let data_size = num_elements * word_size;
            let iso_size = constant::SIZE_HEADER_WRITE + data_size;

            // Setup the telegram
            let mut request_data = transport::READ_WRITE_TELEGRAM.to_vec();
            // Whole telegram Size
            BigEndian::write_u16(request_data[2..].as_mut(), iso_size as u16);
            // Data length
            let mut length = data_size + 4;
            BigEndian::write_u16(request_data[15..].as_mut(), length as u16);
            // Function
            request_data[17] = 0x05;
            // Set DB Number
            request_data[27] = area as u8;

            match area {
                Area::DataBausteine => {
                    BigEndian::write_u16(request_data[25..].as_mut(), db_number as u16)
                }
                _ => {}
            }
            // Adjusts start and word length
            let mut address = match word_len {
                constant::WL_BIT | constant::WL_COUNTER | constant::WL_TIMER => {
                    length = data_size;
                    request_data[22] = word_len as u8;
                    start
                }
                _ => {
                    length = data_size << 3;
                    start << 3
                }
            };

            // Num elements
            BigEndian::write_u16(request_data[23..].as_mut(), num_elements as u16);
            // address into the PLC
            request_data[30] = (address & 0x0FF) as u8;
            address = address >> 8;
            request_data[29] = (address & 0x0FF) as u8;
            address = address >> 8;
            request_data[28] = (address & 0x0FF) as u8;

            // Transport Size
            match word_len {
                constant::WL_BIT => request_data[32] = constant::TS_RES_BIT as u8,
                constant::WL_COUNTER | constant::WL_TIMER => {
                    request_data[32] = constant::TS_RES_OCTET as u8
                }
                _ => request_data[32] = constant::TS_RES_BYTE as u8, // byte/word/dword etc.
            }
            // length
            BigEndian::write_u16(request_data[33..].as_mut(), length as u16);

            //expand values into array
            request_data.splice(
                35..35,
                buffer[offset as usize..offset as usize + data_size as usize].to_vec(),
            );

            let result = self.transport.send(request_data.as_mut_slice());

            match result {
                Ok(response) => {
                    if response.len() != 22 {
                        return Err(Error::Response {
                            code: error::ISO_INVALID_PDU,
                        });
                    }

                    if response[21] != 0xFF {
                        return Err(Error::CPU {
                            code: response[21] as i32,
                        });
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }

            offset += data_size;
            tot_elements -= num_elements;
            start += num_elements * word_size;
        }
        Ok(())
    }
}

impl<T: Transport> Client<T> {
    /// Starting the CPU from power off,Current configuration is discarded and program processing begins again with the initial values.
    pub fn start(&mut self) -> Result<(), Error> {
        self.cold_warm_start_stop(
            transport::COLD_START_TELEGRAM.as_ref(),
            transport::PDU_START,
            error::CLI_CANNOT_START_PLC,
            transport::PDU_ALREADY_STARTED,
            error::CLI_ALREADY_RUN,
        )
    }

    /// Restarting the CPU without turning the power off, Program processing starts once again where Retentive data is retained.
    pub fn restart(&mut self) -> Result<(), Error> {
        self.cold_warm_start_stop(
            transport::WARM_START_TELEGRAM.as_ref(),
            transport::PDU_START,
            error::CLI_CANNOT_START_PLC,
            transport::PDU_ALREADY_STARTED,
            error::CLI_ALREADY_RUN,
        )
    }

    /// Shut down
    pub fn stop(&mut self) -> Result<(), Error> {
        self.cold_warm_start_stop(
            transport::STOP_TELEGRAM.as_ref(),
            transport::PDU_STOP,
            error::CLI_CANNOT_STOP_PLC,
            transport::PDU_ALREADY_STOPPED,
            error::CLI_ALREADY_STOP,
        )
    }

    /// get plc status
    pub fn plc_status(&mut self) -> Result<CpuStatus, Error> {
        let response = self
            .transport
            .send(transport::PLC_STATUS_TELEGRAM.as_ref())?;

        if response.len() < transport::PLC_STATUS_MIN_RESPONSE {
            return Err(Error::Response {
                code: error::ISO_INVALID_PDU,
            });
        }

        let result = BigEndian::read_u16(response[27..29].as_ref());

        if result != 0 {
            return Err(Error::CPU {
                code: result as i32,
            });
        }

        CpuStatus::from_u8(response[44])
    }

    pub fn cp_info(&mut self) -> Result<CPInfo, Error> {
        let szl = self.read_szl(0x0131, 0x000)?;

        Ok(CPInfo {
            max_pdu_length: BigEndian::read_u16(szl.data[2..].as_ref()),
            max_connections: BigEndian::read_u16(szl.data[4..].as_ref()),
            max_mpi_rate: BigEndian::read_u16(szl.data[6..].as_ref()),
            max_bus_rate: BigEndian::read_u16(szl.data[10..].as_ref()),
        })
    }

    /// get cpu info
    pub fn cpu_info(&mut self) -> Result<CpuInfo, Error> {
        let szl = self.read_szl(0x001C, 0x000)?;

        if szl.data.len() < transport::SZL_MIN_RESPONSE {
            return Err(Error::Response {
                code: error::ISO_INVALID_PDU,
            });
        }

        let module_type_name = match str::from_utf8(szl.data[172..204].as_ref()) {
            Ok(s) => s,
            Err(e) => {
                return Err(Error::InvalidResponse {
                    bytes: szl.data[172..204].to_vec(),
                    reason: e.to_string(),
                })
            }
        };

        let serial_number = match str::from_utf8(szl.data[138..162].as_ref()) {
            Ok(s) => s,
            Err(e) => {
                return Err(Error::InvalidResponse {
                    bytes: szl.data[138..162].to_vec(),
                    reason: e.to_string(),
                })
            }
        };

        let as_name = match str::from_utf8(szl.data[2..26].as_ref()) {
            Ok(s) => s,
            Err(e) => {
                return Err(Error::InvalidResponse {
                    bytes: szl.data[2..26].to_vec(),
                    reason: e.to_string(),
                })
            }
        };

        let copyright = match str::from_utf8(szl.data[104..130].as_ref()) {
            Ok(s) => s,
            Err(e) => {
                return Err(Error::InvalidResponse {
                    bytes: szl.data[104..130].to_vec(),
                    reason: e.to_string(),
                })
            }
        };

        let module_name = match str::from_utf8(szl.data[36..60].as_ref()) {
            Ok(s) => s,
            Err(e) => {
                return Err(Error::InvalidResponse {
                    bytes: szl.data[36..60].to_vec(),
                    reason: e.to_string(),
                })
            }
        };

        Ok(CpuInfo {
            module_type_name: module_type_name.to_string(),
            serial_number: serial_number.to_string(),
            as_name: as_name.to_string(),
            copyright: copyright.to_string(),
            module_name: module_name.to_string(),
        })
    }

    fn read_szl(&mut self, id: u16, index: u16) -> Result<transport::S7SZL, Error> {
        let data_szl = 0;
        let mut offset = 0;
        let seq_out: u16 = 0x0000;

        let mut s7_szlfirst = transport::SZL_FIRST_TELEGRAM.to_vec();

        BigEndian::write_u16(s7_szlfirst[11..].as_mut(), seq_out + 1);
        BigEndian::write_u16(s7_szlfirst[29..].as_mut(), id);
        BigEndian::write_u16(s7_szlfirst[31..].as_mut(), index);

        let mut res = self.transport.send(s7_szlfirst.as_ref())?;

        let validate = |res: &[u8], size: usize| -> Result<(), Error> {
            if res.len() < transport::MIN_SZL_FIRST_TELEGRAM + size {
                return Err(Error::Response {
                    code: error::ISO_INVALID_PDU,
                });
            }

            if BigEndian::read_u16(res[27..].as_ref()) != 0 && res[29] != 0xFF {
                return Err(Error::CPU {
                    code: error::CLI_INVALID_PLC_ANSWER,
                });
            }
            Ok(())
        };

        validate(res.as_ref(), 0)?;

        // Skips extra params (ID, Index ...)
        let mut data_szl = BigEndian::read_u16(res[31..].as_ref()) - 8;

        validate(res.as_ref(), data_szl as usize)?;

        let mut done = res[26] == 0x00;
        // Slice sequence
        let mut seq_in: u8 = res[24];
        let header = transport::SZLHeader {
            length_header: BigEndian::read_u16(res[37..].as_ref()) * 2,
            number_of_data_record: BigEndian::read_u16(res[39..].as_ref()),
        };

        let len = (offset + data_szl) as usize;
        let mut data = vec![0u8; len];

        data[offset as usize..len].copy_from_slice(res[41..41 + data_szl as usize].as_ref());

        let mut szl = transport::S7SZL { header, data };
        offset += data_szl;

        let mut s7szlnext: Vec<u8> = transport::SZL_NEXT_TELEGRAM.to_vec();

        while !done {
            BigEndian::write_u16(s7_szlfirst[11..].as_mut(), seq_out + 1);
            s7szlnext[24] = seq_in;

            res = self.transport.send(s7szlnext.as_ref())?;

            validate(res.as_ref(), 0)?;

            data_szl = BigEndian::read_u16(res[31..].as_ref());
            done = res[26] == 0x00;
            seq_in = res[24];

            szl.data = vec![0u8; len];
            offset += data_szl;
            szl.header.length_header += szl.header.length_header;
        }
        Ok(szl)
    }

    fn cold_warm_start_stop(
        &mut self,
        req: &[u8],
        start_cmp: u8,
        start: i32,
        already_cmp: u8,
        already: i32,
    ) -> Result<(), Error> {
        let response = self.transport.send(req)?;

        if response.len() < transport::TELEGRAM_MIN_RESPONSE {
            return Err(Error::Response {
                code: error::ISO_INVALID_PDU,
            });
        }

        if response[17] != start_cmp {
            return Err(Error::Response { code: start });
        }
        if response[18] == already_cmp {
            return Err(Error::Response { code: already });
        }
        Ok(())
    }
}
