use client::Client;
use status::Status;
use std::fmt;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

mod client;
mod status;
pub struct Server {
    server: TcpListener,
    addr: String,
    clients: Vec<Client>,
    sender: Sender<String>,
    receiver: Receiver<String>
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address in Use: {}", self.addr)
    }
}

impl Server {
    pub fn new(addr: &str) -> Self {
        let (sender, receiver) = channel::<String>();
        Self {
            server: TcpListener::bind(addr).unwrap(),
            addr: addr.to_owned(),
            clients: Vec::<Client>::new(),
            sender: sender,
            receiver: receiver
        }
    }

    pub fn accept(&mut self) {
        loop {
            match self.server.accept() {
                Ok((mut stream, _addr)) => {
                    let mut name = [0 as u8; 120];

                    if stream.read(&mut name).is_err() {continue}

                    let client = Client::new(
                        String::from_utf8(name.to_vec()).unwrap(),
                        stream,
                        self.sender.clone()
                    );
                    self.clients.push(client)
                }

                Err(_) => {}
            }
        }
    }
}
