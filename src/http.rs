use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct HttpResponse {
  status_code: u16,
  status_text: String,
  content_type: String,
  body: Vec<u8>,
}

impl HttpResponse {
  pub fn new(
    status_code: u16,
    status_text: &str,
    content_type: &str,
    body: Vec<u8>
  ) -> Self {
    Self {
      status_code,
      status_text: status_text.to_string(),
      content_type: content_type.to_string(),
      body,
    }
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    let headers = format!(
      "HTTP/1.1 {} {}\r\n\
      Content-Type: {}\r\n\
      Content-Length: {}\r\n\
      \r\n",
      self.status_code,
      self.status_text,
      self.content_type,
      self.body.len()
    );

    let mut response = headers.into_bytes();
    response.extend(&self.body);
    response
  }
}

pub async fn send_error_response(
  client_stream: &mut TcpStream,
  status: u16
) -> Result<(), std::io::Error> {
  let (status_text, message) = match status {
    404 => ("Not Found", "Not Found"),
    403 => ("Forbidden", "Access denied"),
    _ => ("Internal Server Error", "Server Error"),
  };

  let response = HttpResponse::new(
    status,
    status_text,
    "text/plain",
    message.as_bytes().to_vec()
  );
  client_stream.write_all(&response.to_bytes()).await?;
  Ok(())
}
