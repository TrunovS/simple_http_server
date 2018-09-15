use std::net::{TcpListener, TcpStream };
use std::io::prelude::*;
use std::fs::File;
use std::thread;
use std::time::Duration;

extern crate thread_pool;
use thread_pool::ThreadPool;
use thread_pool::PoolError;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer));

    let (status_line, filename) = if buffer.starts_with(b"GET /hello HTTP/1.1") {
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    }else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
    };

    let mut file = File::open(filename).expect("Html response file is not opened");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let response = format!("{}{}", status_line,contents);

    // println!("Response: {}", response);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> Result<(),PoolError> {
    let pool = ThreadPool::new(4);
    match pool {
        Err(pool_err) => { return Err(pool_err); },
        Ok(pool) => {
            let lstener = TcpListener::bind("127.0.0.1:7878").unwrap();
            for stream in lstener.incoming().take(3) {
                let stream = stream.unwrap();
                pool.execute(|| {
                    handle_client(stream);
                });
            }
        }
    }
    return Ok(());

}
