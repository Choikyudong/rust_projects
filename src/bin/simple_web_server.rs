use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn main() {
    println!("Server... start");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        thread::spawn(move || {
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();

            let request = String::from_utf8_lossy(&buffer);
            let path = request
                .lines()
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap_or("/");

            let response = match path {
                "/" => "HTTP/1.1 200 OK\r\n\r\nHello World",
                "/hello" => "HTTP/1.1 200 OK\r\n\r\nHello Page",
                "/rust" => "HTTP/1.1 200 OK\r\n\r\nHello Rust",
                _ => "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found"
            };

            stream.write_all(response.as_bytes()).unwrap();
        });
    }
}