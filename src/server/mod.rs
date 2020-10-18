use std::fmt;

pub struct Server<> {
    addr: String,
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address in Use: {}", self.addr)
    }
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.to_owned() }
    }
}
