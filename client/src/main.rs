extern crate short_crypt;

use std::env::args;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::process::exit;
use std::sync::Arc;
use std::thread::{sleep, spawn};
use std::time::Duration;

use short_crypt::ShortCrypt;
fn main() {
    let argv: Vec<String> = args().collect();
    let addr = argv[1]
        .to_owned()
        .to_socket_addrs()
        .expect("Unable to resolve domain name")
        .collect::<Vec<SocketAddr>>()[0];
    let crypter = Arc::new(ShortCrypt::new(&argv[2]));
    let crypter_clone = crypter.clone();
    let mut stream = TcpStream::connect(addr).expect("Couldn't connect to the server");

    println!("Nickname sent successfully, entering into an interactive loop.");

    let mut stream_clone = stream.try_clone().expect("Couldn't clone stream");
    spawn(move || loop {
        let mut buff: Vec<u8> = vec![0 as u8; 1024];

        if stream_clone.read(&mut buff).is_err() {
            println!("Couldn't read from server, exiting");
            exit(1);
        }
        println!("{}", buff.len());
        println!("{}", buff.len());
        buff.retain(|byte| byte != &u8::MIN);
        let buff_string = String::from_utf8(buff).unwrap();
        let decrypted = crypter_clone
            .decrypt_url_component(buff_string)
            .unwrap_or("Couldn't decrypt message".as_bytes().to_vec());

        let decrypted_string = String::from_utf8_lossy(&decrypted);
        println!("{}", decrypted_string);
        sleep(Duration::from_millis(100));
    });

    loop {
        let stdin = std::io::stdin();
        let mut buff = String::from(&argv[3]);
        buff += ": ";
        stdin
            .read_line(&mut buff)
            .expect("Error while trying to read line from stdin");
        let encrypted: Vec<u8> = crypter
            .encrypt_to_url_component(&buff)
            .as_bytes()
            .to_owned();
        if buff.len() > 0 {
            stream
                .write_all(&encrypted)
                .expect("Error while sending message");
        }
        sleep(Duration::from_millis(100));
    }
}
