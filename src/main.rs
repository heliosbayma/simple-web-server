use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::path::{PathBuf};

// Handle incoming connections
fn handle_connection(mut client_stream: TcpStream) {
    // Create a buffer to read the request into, 1024 bytes being enough for basic requests
    let mut request_buffer = [0; 1024];

    // Read the request into the buffer
    if let Ok(_) = client_stream.read(&mut request_buffer) {
        // Convert buffer to string and get the first line
        let http_request = String::from_utf8_lossy(&request_buffer[..]);
        let request_line = http_request.lines().next().unwrap_or("");

        // Parse the path from the request line
        let request_path = request_line
          .split_whitespace()
          .nth(1)
          .unwrap_or("/");

        // Get the file path and sanitize it
        match get_file_path(request_path) {
            Ok(file_path) => {
                match fs::read_to_string(&file_path) {
                    Ok(contents) => {
                        let response = format!(
                            "HTTP/1.1 200 OK\r\n\r\n{}",
                            contents
                        );
                        let _ = client_stream.write_all(response.as_bytes());
                    }
                    Err(_) => {
                        let response = "HTTP/1.1 404 Not Found\r\n\r\nNot Found";
                        let _ = client_stream.write_all(response.as_bytes());
                    }
                }
            }
            Err(e) => {
                // Check the error kind to differentiate between 403 and 404
                let response = match e.kind() {
                    std::io::ErrorKind::NotFound => "HTTP/1.1 404 Not Found\r\n\r\nNot Found",
                    std::io::ErrorKind::PermissionDenied => "HTTP/1.1 403 Forbidden\r\n\r\nAccess denied",
                    _ => "HTTP/1.1 500 Internal Server Error\r\n\r\nServer Error"
                };
                let _ = client_stream.write_all(response.as_bytes());
            }
        }

        let _ = client_stream.flush();
    }
}

fn get_file_path(request_path: &str) -> Result<PathBuf, std::io::Error> {
    // Check for suspicious patterns BEFORE any cleaning
    if request_path.contains("..") ||
       request_path.contains("//") ||
       request_path.contains('\\') {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Invalid path characters detected"
        ));
    }

    // Start with the www directory as the root
    let mut base_path = fs::canonicalize("www")?;

    // Clean the requested path
    let clean_path = request_path.trim_start_matches('/');

    // Handle root path request
    if clean_path.is_empty() {
        base_path.push("index.html");
        return Ok(base_path);
    }

    // Create the full path
    let mut full_path = base_path.clone();
    full_path.push(clean_path);

    // Verify the path is still within www directory
    match fs::canonicalize(&full_path) {
        Ok(canonical_path) => {
            if canonical_path.starts_with(base_path) {
                Ok(full_path)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "Path escapes www directory"
                ))
            }
        }
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Invalid path"
        ))
    }
}

fn main() -> Result<(), std::io::Error> {
    fs::create_dir_all("www")?;

    // Create a TCP listener bound to localhost:80
    let tcp_listener = TcpListener::bind("127.0.0.1:80")?;
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

    Ok(())
}

