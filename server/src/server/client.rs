use super::Status;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::time::SystemTime;
pub struct Client {
    name: String,
    stream: TcpStream,
    sender: Sender<String>,
}

impl Client {
    pub fn new(name: String, stream: TcpStream, sender: Sender<String>) -> Self {
        let instance = Self {
            name,
            stream,
            sender,
        };
        println!(
            "
            \n
            New Client\n
            Name: {}\n
            Address: {}
            ",
            instance.name,
            instance.stream.peer_addr().expect("Could not display addr")
        );
        
        return instance;
    }

    pub fn write(&mut self, message: &str) -> Status {
        match self.stream.write(message.as_bytes()) {
            Ok(_) => Status::Active,
            Err(_) => Status::Inactive,
        }
    }
}
