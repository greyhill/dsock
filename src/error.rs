use std::io;
#[cfg(feature="master")]
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

#[cfg(feature="master")]
impl From<ssh2::Error> for Error {
    fn from(_: ssh2::Error) -> Error {
        Error::SSHError
    }
}
