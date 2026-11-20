//! A minimal static file server for previewing the site: GET and HEAD only,
//! with nothing cached, so that a reload always shows the latest build.

use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;
use std::time::Duration;

use crate::site::OUT;

/// Listens on `addr`: a port on localhost, or `HOST:PORT`.
pub fn bind(addr: &str) -> io::Result<TcpListener> {
    let addr = if addr.contains(':') {
        addr.to_owned()
    } else {
        format!("127.0.0.1:{addr}")
    };
    TcpListener::bind(&addr)
        .map_err(|e| io::Error::new(e.kind(), format!("cannot listen on {addr}: {e}")))
}

/// Serves the output directory, a thread per connection.
pub fn run(listener: TcpListener) {
    for stream in listener.incoming().flatten() {
        thread::spawn(move || handle(stream));
    }
}

fn handle(mut stream: TcpStream) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;
    let request = read_head(&stream)?;
    let mut parts = request.split_whitespace();
    let (method, target) = (
        parts.next().unwrap_or_default(),
        parts.next().unwrap_or("/"),
    );
    let (status, headers, body) = match method {
        "GET" | "HEAD" => respond(target),
        _ => (
            "405 Method Not Allowed",
            "Allow: GET, HEAD\r\n".to_owned(),
            Vec::new(),
        ),
    };
    let length = body.len();
    write!(
        stream,
        "HTTP/1.1 {status}\r\n{headers}Content-Length: {length}\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n"
    )?;
    if method != "HEAD" {
        stream.write_all(&body)?;
    }
    stream.flush()
}

/// Reads a request's head and returns its first line. It reads all of it, as
/// closing a socket with unread input resets the connection, losing the reply.
fn read_head(stream: &TcpStream) -> io::Result<String> {
    let mut reader = BufReader::new(stream);
    let (mut request, mut header) = (String::new(), String::new());
    reader.read_line(&mut request)?;
    while reader.read_line(&mut header)? > 0 && !header.trim_end().is_empty() {
        header.clear();
    }
    Ok(request)
}

/// The status, extra headers and body that answer a GET for `target`.
fn respond(target: &str) -> (&'static str, String, Vec<u8>) {
    let path = target.split(['?', '#']).next().unwrap_or_default();
    let Some(decoded) = decode(path).filter(|path| !path.split('/').any(|part| part == ".."))
    else {
        return ("400 Bad Request", String::new(), Vec::new());
    };
    let mut file = Path::new(OUT).join(decoded.trim_start_matches('/'));
    if file.is_dir() {
        // Pages link relative to their directory, which takes a trailing slash.
        if !path.ends_with('/') {
            return (
                "301 Moved Permanently",
                format!("Location: {path}/\r\n"),
                Vec::new(),
            );
        }
        file.push("index.html");
    }
    match fs::read(&file) {
        Ok(body) => ("200 OK", format!("Content-Type: {}\r\n", mime(&file)), body),
        Err(_) => (
            "404 Not Found",
            "Content-Type: text/plain\r\n".to_owned(),
            b"404 Not Found\n".to_vec(),
        ),
    }
}

/// Decodes a URL path's `%XX` escapes; `None` if they're malformed.
fn decode(path: &str) -> Option<String> {
    let mut bytes = path.bytes();
    let mut decoded = Vec::with_capacity(path.len());
    while let Some(byte) = bytes.next() {
        decoded.push(match byte {
            b'%' => u8::from_str_radix(
                std::str::from_utf8(&[bytes.next()?, bytes.next()?]).ok()?,
                16,
            )
            .ok()?,
            _ => byte,
        });
    }
    String::from_utf8(decoded).ok()
}

fn mime(file: &Path) -> &'static str {
    let extension = file
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();
    match extension.to_ascii_lowercase().as_str() {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "json" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "pdf" => "application/pdf",
        "txt" | "md" | "py" | "r" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_paths() {
        assert_eq!(
            decode("/inf/Zaklady%20Pythonu.html").as_deref(),
            Some("/inf/Zaklady Pythonu.html")
        );
        assert_eq!(decode("/Kru%C5%BEnica").as_deref(), Some("/Kružnica"));
        assert_eq!(decode("/bad%2"), None);
        assert_eq!(decode("/bad%zz"), None);
    }

    #[test]
    fn refuses_to_leave_the_site() {
        for target in [
            "/../Makefile",
            "/mat/%2e%2e/%2E%2E/Makefile",
            "/mat/..%2f..%2fMakefile",
        ] {
            assert_eq!(respond(target).0, "400 Bad Request", "{target}");
        }
    }
}
