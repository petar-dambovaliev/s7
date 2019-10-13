// Copyright 2019 Petar Dambovaliev. All rights reserved.
// This software may be modified and distributed under the terms
// of the BSD license. See the LICENSE file for details.

//! This crate provides communication tools for Siemens s7 family devices
//! # Examples
//! ```no_run
//! # extern crate s7;
//! # use s7::{client::Client, tcp, transport};
//! # use std::time::Duration;
//! # use std::net::{Ipv4Addr, IpAddr};
//!
//! # fn main() {
//!     let addr = Ipv4Addr::new(127, 0, 0, 1);
//!     let mut opts = tcp::Options::new(IpAddr::from(addr), transport::Connection::PG, 5, 5);
//!     opts.read_timeout = Duration::from_secs(2);
//!     opts.write_timeout = Duration::from_secs(2);
//!     let mut cl = match Client::new_tcp(opts) {
//!         Ok(cl) => cl,
//!         Err(e) => {
//!             println!("{:?}", e.to_string());
//!             return;
//!         }
//!     };
//!
//!     let buffer = &mut vec![0u8; 255];
//!
//!     match cl.db_read(1, 1, 3, buffer) {
//!       Ok(()) => println!("buffer: {:?}", buffer),
//!       Err(e) => {}
//!     }
//! # }
//! ```
pub mod client;
mod constant;
pub mod error;
pub mod tcp;
pub mod transport;
