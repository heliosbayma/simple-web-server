mod http;
mod server;
mod utils;

use std::net::TcpListener;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    fs::create_dir_all("www")?;

    let tcp_listener = TcpListener::bind("127.0.0.1:80")?;
    println!("Server listening on port 80");
    println!("Serving files from ./www directory");

    for incoming_stream in tcp_listener.incoming() {
        match incoming_stream {
            Ok(client_stream) => {
                println!("New connection established!");
                if let Err(e) = server::handle_connection(client_stream) {
                    eprintln!("Error handling connection: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
