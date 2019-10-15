# s7
A simple library that can be used to communicate with Siemens S7 family PLC devices

 This crate provides communication tools for Siemens s7 family devices
 So far only `PG.db_read` and `PG.db_write` have been tested on actual hardware
 The crate is unstable as of now and provides no guarantees
 # examples
 ```
extern crate s7;
use s7::{client, tcp, transport};
use std::time::Duration;
use std::net::{Ipv4Addr, IpAddr};

fn main() {
    let addr = Ipv4Addr::new(127, 0, 0, 1);
    let mut opts = tcp::Options::new(IpAddr::from(addr), 5, 5);

    opts.read_timeout = Duration::from_secs(2);
    opts.write_timeout = Duration::from_secs(2);


    let t = tcp::Transport::connect(opts).unwrap();
    let mut cl = client::PG::new(t);

    let buffer = &mut vec![0u8; 1];
    let db = 888;
    let offset = 8.4;
    let size = 1;
    
    cl.db_read(db, offset as i32, size, buffer)?;
    let mut  lights = Bool::new(db, offset, buffer.to_vec())?;
     lights.set_value(!lights.value()); // toggle the light switch
    cl.db_write(lights.data_block(), lights.offset(), Bool::size(), lights.to_bytes().as_mut())?;
}
 ```
# License

Copyright 2019 Petar Dambovaliev. All rights reserved.
This software may be modified and distributed under the terms
of the BSD license. See the LICENSE file for details.
