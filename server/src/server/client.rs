use super::Status;
use std::io::{Write, Read};
use std::net::{TcpStream, Shutdown};
use std::sync::mpsc::{Receiver, Sender};
use std::time::SystemTime;
use std::clone::Clone;

use std::time::Duration;

#[derive(Debug)]
pub struct Client {
    pub stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        let instance = Self {
            stream
        };

        instance.stream.set_read_timeout(Some(Duration::from_millis(300))).unwrap();

        println!(
            "
            \n
            New Client\n
            Address: {}
            ",
            instance.stream.peer_addr().expect("Could not display addr")
        );
        
        return instance;
    }

}
