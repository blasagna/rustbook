extern crate threadpool;

use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::File;

use threadpool::ThreadPool;

const HOST: &'static str = "127.0.0.1:8080";
const BUFSIZE: usize = 512;

fn main() {
    // FIXME: improve error handling
    let listener = TcpListener::bind(HOST).unwrap();

    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);
    println!("Listening on {}...", HOST);

    // iterate over connection attempts
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; BUFSIZE];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let response = format!("{}\r\n\r\n{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
