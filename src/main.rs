mod http;
mod server;
mod utils;

use std::fs;
use tokio::net::TcpListener;

const BIND_ADDRESS: &str = "127.0.0.1:80";
const WWW_DIR: &str = "./www";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  fs::create_dir_all(WWW_DIR)?;

  let tcp_listener = TcpListener::bind(BIND_ADDRESS).await?;
  println!("Server listening on port 80");
  println!("Serving files from {} directory", WWW_DIR);

  loop {
    let (socket, _) = tcp_listener.accept().await?;

    // Spawn a new task to handle the connection
    tokio::spawn(async move {
      if let Err(e) = server::handle_connection(socket).await {
        eprintln!("Error handling connection: {}", e);
      }
    });
  }
}
