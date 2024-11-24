use std::fs;
use std::path::PathBuf;
use std::time::Duration;

pub const MAX_REQUEST_SIZE: usize = 8192;
pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

pub fn get_file_path(request_path: &str) -> Result<PathBuf, std::io::Error> {
  // Check for suspicious patterns BEFORE any cleaning
  if
    request_path.contains("..") ||
    request_path.contains("//") ||
    request_path.contains('\\')
  {
    return Err(
      std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "Invalid path characters detected"
      )
    );
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
      if !canonical_path.starts_with(base_path) {
        return Err(
          std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Path escapes www directory"
          )
        );
      }

      // Check file size if it exists
      if let Ok(metadata) = fs::metadata(&full_path) {
        if metadata.len() > MAX_FILE_SIZE {
          return Err(
            std::io::Error::new(std::io::ErrorKind::Other, "File too large")
          );
        }
      }

      Ok(full_path)
    }
    Err(_) =>
      Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Invalid path")),
  }
}
