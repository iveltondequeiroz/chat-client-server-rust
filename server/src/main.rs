use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;

const LOCAL_HOST: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    println!("Chat Server");
}
