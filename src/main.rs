// Uncomment this block to pass the first stage
use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buffer = [0; 1024];
                let read_size = _stream.read( &mut buffer).unwrap_or_default();
                println!("Read data size : {read_size}");
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                let _ = _stream.write(response.as_bytes()).expect("Tried to send data!");
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
