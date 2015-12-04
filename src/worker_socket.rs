use error::Error;
use std::net;
use std::env;
use std::io::Write;

pub fn get_master_stream() -> Result<(net::TcpStream, u32), Error> {
    let mut args = env::args();
    args.next();

    let hostname: String = args.next().unwrap();
    let port: u16 = args.next()
                        .unwrap().parse()
                        .ok()
                        .unwrap();
    let secret: u8 = args.next()
                         .unwrap()
                         .parse()
                         .ok()
                         .unwrap();
    let id: u32 = args.next()
                      .unwrap()
                      .parse()
                      .ok()
                      .unwrap();

    let mut stream = try!(net::TcpStream::connect((&hostname[..], port)));
    let sbuf: [u8; 1] = [secret];
    try!(stream.write_all(&sbuf[..]));
    try!(stream.flush());

    Ok((stream, id))
}
