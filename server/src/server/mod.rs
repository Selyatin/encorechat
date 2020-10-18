use std::fmt;
use std::net::TcpListener;
use client::Client;
use status::Status;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};


mod client;
mod status;
pub struct Server<> {
    server: TcpListener,
    addr: String,
    clients: Vec<Client>
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address in Use: {}", self.addr)
    }
}

impl Server {
    pub fn new(&self, addr: &str) -> Self {
        Self { server: TcpListener::bind(addr).unwrap(), addr: addr.to_owned(), clients: Vec::<Client>::new() }
    }

    pub fn accept(&self){
        loop {
            match self.server.accept() {
                Ok((stream, addr)) => {}
            }
        }
    }
}
