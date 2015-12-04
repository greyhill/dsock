use std::io;
use ssh2;

#[derive(Debug)]
pub enum Error {
    IOError,
    SSHError,
    WorkerError,
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Error {
        Error::IOError
    }
}

impl From<ssh2::Error> for Error {
    fn from(_: ssh2::Error) -> Error {
        Error::SSHError
    }
}
