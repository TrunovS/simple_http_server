use std::net::{TcpListener, TcpStream };
use std::io::prelude::*;
use std::io;
use std::fs::File;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer));

    let (status_line, filename) = if buffer.starts_with(b"GET / HTTP/1.1") {
        ("HTTP/1.1 200 OK\r\n\r\n", "../html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "../html/404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut response = format!("{}{}", status_line,contents);

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
