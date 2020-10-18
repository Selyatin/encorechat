use super::Status;
use std::net::TcpStream;
use std::io::Write;
pub struct Client {
  name: String,
  stream: TcpStream
}

impl Client {
  pub fn new(name: String, stream: TcpStream) -> Self{
    Self{
      name,
      stream
    }
  }

  pub fn write(&self, message: &str) -> Status{
    match self.stream.write(message.as_bytes()) {
      Ok(result) => Status::Active,
      Err(e) => Status::Inactive
    }
  }
}