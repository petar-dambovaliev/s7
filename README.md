# s7
A simple library that can be used to communicate with Siemens S7 family PLC devices

 This crate provides communication tools for Siemens s7 family devices
 So far only `PG.db_read` and `PG.db_write` have been tested on actual hardware
 The crate is unstable as of now and provides no guarantees
 # examples
 ```rust
extern crate s7;

use s7::{client, field::Bool, field::Field, field::Fields, field::Float, tcp};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

fn main() {
    let addr = Ipv4Addr::new(127, 0, 0, 1);
    let mut opts = tcp::Options::new(IpAddr::from(addr), 5, 5);

    opts.read_timeout = Duration::from_secs(2);
    opts.write_timeout = Duration::from_secs(2);

    let t = tcp::Transport::connect(opts).unwrap();
    let mut cl = client::PG::new(t).unwrap();

    let buffer = &mut vec![0u8; Bool::size() as usize];
    let db = 888;

    // the offset in the PLC is represented by a float
    // the difit on the left is the index within the block
    // the digit after the decimal point is only important for the `Bool` to be able to change the relevant bit
    // we don't need after
    let mut  offset = 8.4;

    // Since this is a boolean field, we are going to get back 1 byte
    cl.db_read(db, offset as i32, Bool::size(), buffer).unwrap();

    // field mod provides types to handle the data from the PLC
    // create a bool field from the byte we got
    let mut lights = Bool::new(db, offset, buffer.to_vec()).unwrap();

    // the bit in the byte is set without changing any of the other bits
    lights.set_value(!lights.value()); // toggle the light switch

    offset = 12.0;
    let mut cooling_buffer = vec![0u8; Float::size() as usize];
    cl.db_read(db, offset as i32, Float::size(), cooling_buffer.as_mut())
        .unwrap();
    let mut cooling = Float::new(db, offset, cooling_buffer).unwrap();
    cooling.set_value(121.3);

    let fields: Fields = vec![Box::new(lights), Box::new(cooling)];

    // save back the changed values
    for field in fields.iter() {
        cl.db_write(
            field.data_block(),
            field.offset(),
            field.to_bytes().len() as i32,
            field.to_bytes().as_mut(),
        )
        .unwrap();
    }
}
 ```
# License

Copyright 2019 Petar Dambovaliev. All rights reserved.
This software may be modified and distributed under the terms
of the BSD license. See the LICENSE file for details.
