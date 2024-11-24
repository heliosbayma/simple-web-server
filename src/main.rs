mod http;
mod server;
mod utils;

use std::net::TcpListener;
use std::fs;
use std::thread;

const BIND_ADDRESS: &str = "127.0.0.1:80";
const WWW_DIR: &str = "./www";

fn main() -> Result<(), std::io::Error> {
  fs::create_dir_all(WWW_DIR)?;

  let tcp_listener = TcpListener::bind(BIND_ADDRESS)?;
  println!("Server listening on port 80");
  println!("Serving files from {} directory", WWW_DIR);

  for incoming_stream in tcp_listener.incoming() {
    match incoming_stream {
      Ok(client_stream) => {
        println!("New connection established!");

        // Spawn a new thread for each connection
        thread::spawn(move || {
          let thread_id = thread::current().id();
          println!("Handling connection in thread: {:?}", thread_id);

          if let Err(e) = server::handle_connection(client_stream) {
            eprintln!(
              "Error handling connection in thread {:?}: {}",
              thread_id,
              e
            );
          }
        });
      }
      Err(e) => {
        eprintln!("Error accepting connection: {}", e);
      }
    }
  }

  Ok(())
}
