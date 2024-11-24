use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::path::{PathBuf};

// Handle incoming connections
fn handle_connection(mut client_stream: TcpStream) {
    let mut request_buffer = [0; 4096];

    if let Ok(bytes_read) = client_stream.read(&mut request_buffer) {
        // Convert buffer to string using only the bytes that were read
        let http_request = String::from_utf8_lossy(&request_buffer[..bytes_read]);
        let mut lines = http_request.lines();

        // Parse request line
        let request_line = lines.next().unwrap_or("");
        let mut request_parts = request_line.split_whitespace();
        request_parts.next(); // Skip the method
        let request_path = request_parts.next().unwrap_or("/");

        // Parse headers
        let mut headers = std::collections::HashMap::new();
        while let Some(line) = lines.next() {
            if line.is_empty() { break; }  // Empty line separates headers from body
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(
                    key.trim().to_lowercase(),
                    value.trim().to_string()
                );
            }
        }

        // Handle the request
        match get_file_path(request_path) {
            Ok(file_path) => {
                match fs::read_to_string(&file_path) {
                    Ok(contents) => {
                        let response = format!(
                            "HTTP/1.1 200 OK\r\n\
                            Content-Type: text/html\r\n\
                            Content-Length: {}\r\n\
                            \r\n\
                            {}",
                            contents.len(),
                            contents
                        );
                        let _ = client_stream.write_all(response.as_bytes());
                    }
                    Err(_) => send_error_response(&mut client_stream, 404)
                }
            }
            Err(e) => {
                let status = match e.kind() {
                    std::io::ErrorKind::NotFound => 404,
                    std::io::ErrorKind::PermissionDenied => 403,
                    _ => 500
                };
                send_error_response(&mut client_stream, status);
            }
        }

        let _ = client_stream.flush();
    }
}

fn send_error_response(client_stream: &mut TcpStream, status: u16) {
    let (status_line, message) = match status {
        404 => ("404 Not Found", "Not Found"),
        403 => ("403 Forbidden", "Access denied"),
        _ => ("500 Internal Server Error", "Server Error")
    };

    let response = format!(
        "HTTP/1.1 {}\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: {}\r\n\
        \r\n\
        {}",
        status_line,
        message.len(),
        message
    );
    let _ = client_stream.write_all(response.as_bytes());
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

