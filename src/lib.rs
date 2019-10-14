// Copyright 2019 Petar Dambovaliev. All rights reserved.
// This software may be modified and distributed under the terms
// of the BSD license. See the LICENSE file for details.

//! This crate provides communication tools for Siemens s7 family devices
//! So far only `PG.db_read` and `PG.db_write` have been tested on actual hardware
//! The crate is unstable as of now and provides no guarantees
//! # Examples
//! ```no_run
//! # extern crate s7;
//! # use s7::{client, tcp, transport};
//! # use std::time::Duration;
//! # use std::net::{Ipv4Addr, IpAddr};
//!
//! # fn main() {
//! let addr = Ipv4Addr::new(127, 0, 0, 1);
//! let mut opts = tcp::Options::new(IpAddr::from(addr), 5, 5);
//!
//! opts.read_timeout = Duration::from_secs(2);
//! opts.write_timeout = Duration::from_secs(2);
//!
//!
//! let t = tcp::Transport::connect(opts).unwrap();
//! let mut cl = client::PG::new(t);
//!
//!     let buffer = &mut vec![0u8; 255];
//!
//!     match cl.db_read(888, 8, 1, buffer) {
//!       Ok(()) => println!("buffer: {:?}", buffer),
//!       Err(e) => println!("error: {:?}", e),
//!     }
//! # }
//! ```
pub mod client;
mod constant;
pub mod error;
pub mod tcp;
pub mod transport;
