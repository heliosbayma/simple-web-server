# Basic HTTP Server in Rust

A weekend exercise exploring Rust for the first time.
Inspired by [Web Server Coding Challenge](https://codingchallenges.fyi/challenges/challenge-webserver/)

## Features

Step 1:

- TCP server listening on localhost (127.0.0.1) port 80
- Handles basic HTTP GET requests
- Returns the requested path in the response
- Single-threaded, handling one connection at a time

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

2. In another terminal, you can test the server using curl:

```bash
curl http://localhost/
curl http://localhost/hello
curl http://localhost/test/path
```

Example output:

```text
Requested path: /
Requested path: /hello
Requested path: /test/path
```

## Project Structure

```text
http_server/
├── src/
│   └── main.rs      # Server implementation
├── Cargo.toml       # Project dependencies and metadata
└── README.md        # This file
```

## Current Limitations (by design)

- Handles only one connection at a time
- Only processes GET requests
- No support for HTTP headers or request bodies
- No proper error handling or logging
- No configuration options (fixed to localhost:80)

## Future Improvements

see [Web Server Coding Challenge](https://codingchallenges.fyi/challenges/challenge-webserver/)