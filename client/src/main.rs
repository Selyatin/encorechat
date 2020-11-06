use std::env::args;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::thread::spawn;
use std::process::exit;
use std::net::ToSocketAddrs;
use std::net::SocketAddr;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let argv: Vec<String> = args().collect();
    let addr = argv[1].to_owned().to_socket_addrs().expect("Unable to resolve domain name").collect::<Vec<SocketAddr>>()[0];
    let mut stream = TcpStream::connect(addr).expect("Couldn't connect to the server");

    println!("Nickname sent successfully, entering into an interactive loop.");
    
    let mut stream_clone = stream.try_clone().expect("Couldn't clone stream");
    spawn(move || {
        loop {
            let mut buff: Vec<u8> = vec![0; 512];

            let err = stream_clone.read_exact(&mut buff).is_err();

            if err {
                println!("Couldn't read from server, exiting");
                exit(1);
            }

            println!("{}", String::from_utf8(buff).unwrap_or(String::from("Couldn't unwrap message")));
            sleep(Duration::from_millis(100));
        }
    });

    loop {
        let stdin = std::io::stdin();
        let mut buff = String::from(&argv[2]);
        buff += ": ";
        stdin
            .read_line(&mut buff)
            .expect("Error while trying to read line from stdin");
        if buff.len() > 0 {
            stream
                .write_all(buff.as_bytes())
                .expect("Error while sending message");
        }

        sleep(Duration::from_millis(100));
    }
}
