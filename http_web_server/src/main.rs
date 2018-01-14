use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::File;

const HOST: &'static str = "127.0.0.1:8080";
const BUFSIZE: usize = 512;

fn main() {
    // FIXME: improve error handling
    let listener = TcpListener::bind(HOST).unwrap();
    println!("Listening on {}...", HOST);

    // iterate over connection attempts
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; BUFSIZE];

    stream.read(&mut buffer).unwrap();
        
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let mut file = File::open("hello.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
