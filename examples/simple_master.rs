extern crate dsock;

use std::env;
use std::io::Read;

pub fn main() -> () {
    let mut args = env::args();
    args.next();

    let my_addr = args.next().unwrap();
    let host = args.next().unwrap();
    let username = args.next().unwrap();
    let path = args.next().unwrap();

    let mut m = dsock::MasterNode::new(my_addr, 8888).ok().expect("error creating MasterNode");
    let mut stream = m.connect((&host[..], 22u16), &username, &path)
                      .ok()
                      .expect("error getting connection to worker");

    let mut b: [u8; 1] = [0];
    stream.read(&mut b[..]).ok().expect("error reading from worker");
    println!("Message from worker: {:?}", b[0]);
}
