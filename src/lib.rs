/// This package provides a very basic framework for distributed computing
/// using a small number of nodes.  It mostly provides a utility for a master
/// program to copy a binary to a remote machine over SSH, then launch that
/// binary and open a TCP connection to it.
///
/// The basic code in the master binary is:
/// ```
/// let port = 8080u16;
/// let master = MasterNode(port).ok().unwrap();
/// let worker: TcpStream = master.connect(("remote_host", 22u16), "username", "/tmp/worker_binary").ok.unwrap();
/// ```
/// This connects to the SSH server running on `remote_host:22`, copies over the
/// binary located at `/tmp/worker_binary` on the master node, sets a few environment
/// variables, then launches the binary.
///
/// The worker code is simple too:
/// ```
/// fn main() {
///     let master: TcpStream = get_master_stream().ok().unwrap();
/// }
/// ```
///
/// That's all there is to it.  If you want anything more advanced, you'll need
/// to build on these sockets :)
#[macro_use]
extern crate log;
extern crate ssh2;
extern crate rand;

mod master_socket;
mod worker_socket;
mod error;

pub use error::Error;
pub use master_socket::MasterNode;
pub use worker_socket::get_master_stream;
