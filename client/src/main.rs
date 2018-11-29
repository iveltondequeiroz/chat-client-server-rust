use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mspc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL_HOST: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;


fn main() {
    let mut client = TcpStream::connect(LOCAL_HOST).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initialize non-blocking");
}
