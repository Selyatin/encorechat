use std::net::{TcpStream, TcpListener, Shutdown};
use std::vec::Vec;
use std::sync::{mpsc::{channel}, Arc, Mutex};
use std::env::args;
use std::thread::{spawn};
use std::io::{Read, Write};

fn sleep(){
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn main(){
    let args: Vec<String> = args().collect();
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &args[1]).expect("Couldn't bind");
    listener.set_nonblocking(true).expect("Couldn't set nonblocking");
    let clients_lock = Arc::new(Mutex::new(Vec::<TcpStream>::new()));
    let clients_lock_clone = clients_lock.clone();
    let (sender, receiver) = channel::<Vec<u8>>();
    println!("---- Encorechat ----");
    loop {
        println!("Clients: {:?}", clients_lock.lock().expect("Couldn't lock client_lock in println"));
        if let Ok((mut stream, _)) = listener.accept(){
            let mut clients = clients_lock.lock().expect("Couldn't lock clients_lock");
            clients.push(stream.try_clone().expect("Couldn't clone stream to put it inside clients Vector"));
            let sender_clone = sender.clone();

            spawn(move || loop {
                let mut buff: Vec<u8> = vec![0 as u8; 2056];
    
                if stream.read(&mut buff).is_err(){
                    println!("Error while reading from stream: {:?}", stream);
                    break
                }

                if buff[0] != 0 {
                    sender_clone.send(buff);
                }

                sleep();
            });
        }

        if let Ok(msg) = receiver.try_recv() {
            let mut clients = clients_lock_clone.lock().expect("Couldn't lock client_lock in receiver");
            let i = 0;
            while i < clients.len(){
                let mut client = &mut clients[i];
                match client.write(&msg) {
                    Ok(_) => {
                        continue;
                    },
                    
                    Err(_) => {
                        clients.remove(i);
                    }
                }
            }
        }

    sleep();    
    }
}
