use std::io::{ Read, Write };
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

use crate::http::{ HttpResponse, send_error_response };
use crate::utils::{ MAX_REQUEST_SIZE, CONNECTION_TIMEOUT, get_file_path };

pub fn handle_connection(
  mut client_stream: TcpStream
) -> Result<(), std::io::Error> {
  // Get and log the thread ID
  let thread_id = thread::current().id();
  let start_time = SystemTime::now();
  // end of logging

  // Set timeout
  client_stream.set_read_timeout(Some(CONNECTION_TIMEOUT))?;
  client_stream.set_write_timeout(Some(CONNECTION_TIMEOUT))?;

  let mut request_buffer = [0; MAX_REQUEST_SIZE];

  let bytes_read = client_stream.read(&mut request_buffer)?;

  // Convert buffer to string using only the bytes that were read
  let http_request = String::from_utf8_lossy(&request_buffer[..bytes_read]);
  let mut lines = http_request.lines();

  // Parse request line
  let request_line = lines.next().unwrap_or("");
  let mut request_parts = request_line.split_whitespace();
  request_parts.next(); // Skip the method
  let request_path = request_parts.next().unwrap_or("/");

  println!(
    "[{:?}] [{:?}] Starting new request",
    start_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
    thread_id
  );

  // Add the 5-second delay (uncomment lines below for testing)
  // println!("Thread {:?} sleeping for 5 seconds...", thread_id);
  // thread::sleep(Duration::from_secs(5));
  // println!("Thread {:?} woke up, processing request...", thread_id);

  // Parse headers
  let mut headers = std::collections::HashMap::new();
  while let Some(line) = lines.next() {
    if line.is_empty() {
      break;
    }
    if let Some((key, value)) = line.split_once(':') {
      headers.insert(key.trim().to_lowercase(), value.trim().to_string());
    }
  }

  // Handle the request
  match get_file_path(request_path) {
    Ok(file_path) => {
      match fs::read(&file_path) {
        Ok(contents) => {
          let response = HttpResponse::new(200, "OK", "text/html", contents);
          client_stream.write_all(&response.to_bytes())?;
        }
        Err(_) => send_error_response(&mut client_stream, 404)?,
      }
    }
    Err(e) => {
      let status = match e.kind() {
        std::io::ErrorKind::NotFound => 404,
        std::io::ErrorKind::PermissionDenied => 403,
        _ => 500,
      };
      send_error_response(&mut client_stream, status)?;
    }
  }

  println!(
    "[{:?}] [{:?}] Request completed in {:?}",
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
    thread_id,
    SystemTime::now().duration_since(start_time).unwrap()
  );

  client_stream.flush()?;
  Ok(())
}
