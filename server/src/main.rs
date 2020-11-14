use std::env::args;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread::spawn;
use std::vec::Vec;

fn sleep() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn main() {
    let args: Vec<String> = args().collect();

    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &args[1]).expect("Couldn't bind");

    listener
        .set_nonblocking(true)
        .expect("Couldn't set nonblocking");

    let clients_lock = Arc::new(Mutex::new(Vec::<TcpStream>::new()));
    let clients_lock_clone = clients_lock.clone();

    let (sender, receiver) = channel::<Vec<u8>>();

    println!("---- Encorechat ----");

    loop {
        if let Ok((mut stream, _)) = listener.accept() {
            
            println!("New Client: {:?}", stream);
            
            let mut clients = clients_lock.lock().expect("Couldn't lock clients_lock");

            clients.push(
                stream
                    .try_clone()
                    .expect("Couldn't clone stream to put it inside clients Vector"),
            );
            
            let sender_clone = sender.clone();
            
            spawn(move || loop {
                let mut buff: Vec<u8> = vec![0 as u8; 512];

                if stream.read(&mut buff).is_err() {
                    println!("Error while reading from stream: {:?}", stream);
                    break;
                }

                buff.retain(|byte| byte != &u8::MIN);

                if buff.len() == 0 {
                    break;
                }

                println!("{}", String::from_utf8_lossy(&buff));
                sender_clone.send(buff).expect("Sender failed");

                sleep();
            });
        }

        if let Ok(msg) = receiver.try_recv() {
            let mut clients = clients_lock_clone
                .lock()
                .expect("Couldn't lock client_lock in receiver");

            let mut i = 0;

            while i < clients.len() {
                let client = &mut clients[i];
                if client.write(&msg).is_err() {
                    clients.remove(i);
                }
                i += 1;
            }
        }

        sleep();
    }
}
