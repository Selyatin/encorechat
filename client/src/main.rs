use std::env::args;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
fn main() {
    let argv: Vec<String> = args().collect();

    let mut stream = TcpStream::connect("127.0.0.1:8081").expect("Couldn't connect to the server");

    if stream.write_all(argv[1].as_bytes()).is_err() {
        return println!("Couldn't send nickname... Terminating.");
    }

    println!("Nickname sent successfully, entering into an interactive loop.");

    loop {
        let stdin = std::io::stdin();
        let mut buff = String::new();
        stdin
            .read_line(&mut buff)
            .expect("Error while trying to read line from stdin");
        if buff.len() > 0 {
            stream
                .write_all(buff.as_bytes())
                .expect("Error while sending message");
        }
    }
}
