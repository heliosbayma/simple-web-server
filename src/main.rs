mod http;
mod server;
mod utils;

use std::net::TcpListener;
use std::fs;
use threadpool::ThreadPool;

const BIND_ADDRESS: &str = "127.0.0.1:80";
const WWW_DIR: &str = "./www";

fn main() -> Result<(), std::io::Error> {
  fs::create_dir_all(WWW_DIR)?;

  let tcp_listener = TcpListener::bind(BIND_ADDRESS)?;
  println!("Server listening on port 80");
  println!("Serving files from {} directory", WWW_DIR);

  let pool = ThreadPool::new(4);

  for incoming_stream in tcp_listener.incoming() {
    let client_stream = incoming_stream?;
    pool.execute(move || {
      if let Err(e) = server::handle_connection(client_stream) {
        eprintln!("Error handling connection: {}", e);
      }
    });
  }

  Ok(())
}
