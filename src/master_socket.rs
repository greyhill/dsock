use error::Error;

use std::net;
use std::path;
use std::fs;
use std::io::{Read, Write};
use rand;
use ssh2;

pub struct MasterNode {
    hostname: String,
    port: u16,
    listener: net::TcpListener,
    session_store: Vec<(net::TcpStream, ssh2::Session)>,
}

impl MasterNode {
    pub fn new(hostname: String, port: u16) -> Result<MasterNode, Error> {
        let listener = try!(net::TcpListener::bind(("0.0.0.0", port)));
        Ok(MasterNode {
            hostname: hostname,
            port: port,
            listener: listener,
            session_store: Vec::new(),
        })
    }

    fn accept_secret(self: &Self, secret: u8) -> Result<net::TcpStream, Error> {
        loop {
            match self.listener.accept() {
                Ok((mut stream, _)) => {
                    let mut b: [u8; 1] = [0];
                    if !stream.read(&mut b[..]).ok().is_some() {
                        continue;
                    }
                    if b[0] == secret {
                        return Ok(stream);
                    }
                }
                Err(_) => {
                    // keep trying
                }
            }
        }
    }

    pub fn connect<A: net::ToSocketAddrs, P: AsRef<path::Path>, S: AsRef<str>>
                                                                               (self: &mut Self,
                                                                                addr: A,
                                                                                username: S,
                                                                                bin_path: P)
                                                                                -> Result<net::TcpStream, Error> {
        let secret: u8 = rand::random();

        // open TCP socket to remote host; create an SSH session and
        // authenticate using given username and password
        let ssh_socket = try!(net::TcpStream::connect(addr));
        let mut ssh_session = ssh2::Session::new().expect("error creating SSH session");
        try!(ssh_session.handshake(&ssh_socket));
        try!(ssh_session.userauth_agent(username.as_ref()));

        // open the local file specified by bin_path
        let mut bin_contents: Vec<u8> = Vec::new();
        let mut bin_file = try!(fs::File::open(bin_path));
        let bin_size = try!(bin_file.read_to_end(&mut bin_contents));

        // send file to remote host at /tmp/dsock_binary
        // TODO: support windows, use a smarter path?
        {
            let mut remote_file = try!(ssh_session.scp_send(path::Path::new("/tmp/dsock_binary"),
                                                            0o700,
                                                            bin_size as u64,
                                                            None));
            try!(remote_file.write_all(&bin_contents[..]));
            try!(remote_file.flush());
        }

        // start a new channel and set environment variables
        {
            let worker_id = self.session_store.len() as u32;
            // run the binary we copied over earlier
            let mut channel = try!(ssh_session.channel_session());
            try!(channel.exec(&format!("/tmp/dsock_binary \"{}\" \"{}\" \"{}\" \"{}\"",
                                       self.hostname,
                                       self.port,
                                       secret,
                                       worker_id)[..]));
        }


        // accept TCP connections until we find one with the secret we sent
        // earlier
        let tcp_socket = try!(self.accept_secret(secret));

        self.session_store.push((ssh_socket, ssh_session));

        Ok(tcp_socket)
    }
}
