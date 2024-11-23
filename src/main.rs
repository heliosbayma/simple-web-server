use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::path::{Path, PathBuf};

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

    // Get the file path and sanitize it
    let file_path = get_file_path(request_path);

    match serve_file(&file_path) {
      Ok(contents) => {
          let response = format!(
              "HTTP/1.1 200 OK\r\n\r\n{}",
              contents
          );
          client_stream.write_all(response.as_bytes()).unwrap();
      }
      Err(_) => {
          let response = "HTTP/1.1 404 Not Found\r\n\r\nNot Found";
          client_stream.write_all(response.as_bytes()).unwrap();
        }
    }

    client_stream.flush().unwrap();
}

fn get_file_path(request_path: &str) -> PathBuf {
    let mut path = PathBuf::from("www");

    let clean_path = request_path.trim_start_matches('/');

    // Following common web server conventions
    if clean_path.is_empty() {
        path.push("index.html");
    } else {
        path.push(clean_path);
    }
    if path.is_dir() {
        path.push("index.html");
    }

    path
}

fn serve_file(file_path: &Path) -> std::io::Result<String> {
    let canonical_path = fs::canonicalize(file_path)?;
    let www_path = fs::canonicalize("www")?;

    if !canonical_path.starts_with(www_path) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Access denied"
        ));
    }

    fs::read_to_string(file_path)
}

fn main() {
    fs::create_dir_all("www").unwrap();

    // Create a TCP listener bound to localhost:80
    let tcp_listener = TcpListener::bind("127.0.0.1:80").unwrap();
    println!("Server listening on port 80");
    println!("Serving files from ./www directory");

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

