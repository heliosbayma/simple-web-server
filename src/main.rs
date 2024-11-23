use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// TODO:
// - Replace unwrap with proper error handling

// Handle incoming connections
fn handle_connection(mut client_stream: TcpStream) {
    // Create a buffer to read the request into, 1024 bytes being enough for basic requests
    let mut request_buffer = [0; 1024];

    // Read the request into the buffer
    client_stream.read(&mut request_buffer).unwrap();

    // Convert buffer to string and get the first line
    let http_request = String::from_utf8_lossy(&request_buffer[..]);
    let request_line = http_request.lines().next().unwrap_or("");

    // Parse the path from the request line
    let request_path = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("/");

    // Prepare the response
    let http_response = format!(
        "HTTP/1.1 200 OK\r\n\r\nRequested path: {}\r\n",
        request_path
    );

    // Send the response
    client_stream.write_all(http_response.as_bytes()).unwrap();
    client_stream.flush().unwrap();
}

fn main() {
    // Create a TCP listener bound to localhost:80
    let tcp_listener = TcpListener::bind("127.0.0.1:80").unwrap();
    println!("Server listening on port 80");

    // Handle incoming connections
    for incoming_stream in tcp_listener.incoming() {
        match incoming_stream {
            Ok(client_stream) => {
                println!("New connection established!");
                handle_connection(client_stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}