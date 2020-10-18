use server::Server;
mod server;

fn main() {
  let mut server = Server::new("127.0.0.1:8081");
  
  // Start accepting connections and process them
  server.accept();
}