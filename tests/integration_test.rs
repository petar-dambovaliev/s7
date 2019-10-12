extern crate s7;
use s7::{client::Client, tcp};
use std::time::Duration;

#[test]
fn test_client() {
    let mut opts = tcp::Options::new("127.0.0.1:9999".to_string(), tcp::Connection::PG, 5, 5);
    opts.read_timeout = Duration::from_secs(2);
    opts.write_timeout = Duration::from_secs(2);
    let mut cl = Client::new_tcp(opts).unwrap();

    let buffer = &mut vec![0u8; 25];

    match cl.db_read(1, 1, 3, buffer) {
        Ok(()) => {}
        Err(e) => {}
    }
}
