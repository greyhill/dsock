extern crate dsock;

use std::io::Write;

pub fn main() -> () {
    let (mut stream, id) = dsock::get_master_stream().unwrap();
    let b: [u8; 1] = [42];
    stream.write_all(&b[..]).ok().expect("error writing");
    stream.flush().ok().expect("error flushing");
}
