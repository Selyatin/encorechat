use client::Client;
use status::Status;
use std::fmt;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{spawn, JoinHandle, Thread};

mod client;
mod status;
pub struct Server {
    server: TcpListener,
    addr: String,
    clients: Vec<Sender<String>>,
    sender: Sender<String>,
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
            clients: Vec::new(),
            sender: sender,
        }
    }

    pub fn accept(mut self) {
        spawn(move || loop {
            match self.server.accept() {
                Ok((mut stream, _addr)) => {
                    let client: Client = Client::new(stream);

                    let mut reader_stream = client.stream.try_clone().unwrap();
                    let mut writer_stream = client.stream;

                    let (sender, receiver) = channel::<&[u8]>();

                    spawn(move || loop {
                        match receiver.recv() {
                            Ok(message) => {
                                writer_stream.write(message);
                            }

                            Err(_) => {
                                println!("[-] A Client has disconnected");
                                break;
                            }
                        }
                    });

                    spawn(move || {
                        let sender = sender.clone();
                        loop {
                            let mut read_buffer = [0 as u8; 512];

                            let read_err = reader_stream.read(&mut read_buffer).is_err();
                            if read_err {
                                println!("Connection lost with a client");
                                break
                            }
                            
                            sender.send(&read_buffer);

                            // match reader_stream.read(&mut read_buffer) {
                            //     Ok(_) => {
                            //         let err = sender.send(&read_buffer).is_err();
                            //         if err {break}
                            //     }

                            //     Err(_) => {
                            //         println!("[-] A Client has disconnected");
                            //         break;
                            //     }
                            // };
                        }
                    });
                }

                Err(_) => {}
            }
        })
        .join()
        .unwrap()
    }
}
