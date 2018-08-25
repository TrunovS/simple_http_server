use std::net::{TcpListener, TcpStream };
use std::io::prelude::*;
use std::io;
use std::fs::File;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer));

    let mut contents = String::new();
    let mut response = String::new();

    if buffer.starts_with(b"GET / HTTP/1.1") {
        let mut file = File::open("../html/hello.html").unwrap();
        file.read_to_string(&mut contents).unwrap();
        response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    } else
    {
        let mut file = File::open("../html/404.html").unwrap();
        file.read_to_string(&mut contents).unwrap();
        response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents);
    }

    println!("Response: {}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> io::Result<()> {
    let lstener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in lstener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
