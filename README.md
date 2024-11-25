# Basic HTTP Server in Rust

A weekend exercise exploring Rust for the first time.
Inspired by [Web Server Coding Challenge](https://codingchallenges.fyi/challenges/challenge-webserver/)

## Features

Step 1:

- TCP server listening on localhost (127.0.0.1) port 80
- Handles basic HTTP GET requests
- Returns the requested path in the response
- Single-threaded, handling one connection at a time

Step 2.a:

- Handles static files in the `www` directory
- Adds proper path sanitization of the requested path to avoid directory traversal attacks
- Adds error handling
- Changed approach to handle file requests by handling them together with the root path request
  This simplifies error handling and allow better control over the response and more DRY
  Less cleaner separation of concerns but ok for what we need
- Increases request buffer size to 4096 bytes
- Adds parsing of HTTP headers
- Adds HTTP status codes to responses
- Cleaner approach to path sanitization

Step 2.b (going out of scope for learning purposes):

- Connection timeout
- Added structs and impl blocks for HttpResponse
- Added consts for security limits
- Improved logic for handle_connection: it's getting more readable but still a WIP
- Improved error handling: also a WIP
- Breaking main file into modules as it was getting too big

Step 3

- Handle multiple requests concurrently with three approaches:
  - Using threads spawn (not realistic for production)
  - [TBD] Using a thread pool
  - [TBD] Using Tokio for async operations
- Added logging


## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- Administrative privileges (for binding to port 80)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/heliosbayma/simple-web-server.git
cd http_server
```

2. Build the project:

```bash
cargo build
```

## Usage

1. Run the server (requires administrative privileges for port 80):

```bash
sudo cargo run
```

2. In another terminal, you can test the server using curl. Here are some example requests:

Normal requests:

```bash
# Access the root page
curl -i http://localhost/

# Access non-existent pages
curl -i http://localhost/hello
curl -i http://localhost/test/path
```

Test concurrent requests

```bash
# Make the test script executable
chmod +x scripts/test_concurrent.sh

# Run concurrent test (sends 10 simultaneous requests)
./scripts/test_concurrent.sh
```

Security test requests:

```bash
# Basic path traversal attempt
curl -i http://localhost/../../../etc/passwd

# URL-encoded path traversal
curl -i http://localhost/..%2F..%2F..%2Fetc%2Fpasswd

# Double-encoded path traversal
curl -i http://localhost/%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd

# Mixed path traversal
curl -i http://localhost/static/../../../etc/passwd
```

Example responses:

Successful request:

```text
HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 345

<html>...</html>
```

Missing file:

```text
HTTP/1.1 404 Not Found
Content-Type: text/plain
Content-Length: 9

Not Found
```

Blocked security attempts:

```text
HTTP/1.1 400 Bad Request
Content-Type: text/plain
Content-Length: 39

Path traversal attempts are not permitted
```

## Project Structure

```text
http_server/
├── www/             # Static files
│   └── index.html
├── src/
│   ├── main.rs      # Server entry point
│   ├── http.rs      # HTTP response struct and methods
│   ├── server.rs    # Server implementation
│   └── utils.rs     # Utility functions
├── Cargo.toml       # Project dependencies and metadata
└── README.md        # This file
```

## Current Limitations (by design)

- Handles only one connection at a time
- Only processes GET requests
- No configuration options (fixed to localhost:80)

## Future Improvements

- MIME type mapping (lazy_static! ???)
- Add logging
- Add tests


see [Web Server Coding Challenge](https://codingchallenges.fyi/challenges/challenge-webserver/)
