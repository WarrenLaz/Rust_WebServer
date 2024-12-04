use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Listening on 127.0.0.1:7878...");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let response = if buffer.starts_with(get) {
        let contents = fs::read_to_string("/Users/warren_lazarraga/Programming_projects/Low_Level_Programming/rustweb/web_server/src/index.html").unwrap_or_else(|_| {
            String::from("<h1>File not found. Please add an index.html file.</h1>")
        });
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        )
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = "404 Not Found";
        format!("{}{}", status_line, contents)
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

