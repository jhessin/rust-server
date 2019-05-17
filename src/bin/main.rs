use server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("192.168.5.202:7878").unwrap();
    println!("Listening on port 7878");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting Down...");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let content = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
