mod http;
mod server;
mod utils;

use std::fs;
use tokio::net::TcpListener;

const BIND_ADDRESS: &str = "127.0.0.1:80";
const WWW_DIR: &str = "./www";

fn main() -> Result<(), std::io::Error> {
  // Create runtime
  let runtime = tokio::runtime::Runtime::new()?;

  // Enter the runtime
  runtime.block_on(async {
    fs::create_dir_all(WWW_DIR)?;

    let tcp_listener = TcpListener::bind(BIND_ADDRESS).await?;
    println!("Server listening on port 80");
    println!("Serving files from {} directory", WWW_DIR);

    loop {
      let (client_stream, _) = tcp_listener.accept().await?;

      tokio::spawn(async move {
        if let Err(e) = server::handle_connection(client_stream).await {
          eprintln!("Error handling connection: {}", e);
        }
      });
    }
  })
}
