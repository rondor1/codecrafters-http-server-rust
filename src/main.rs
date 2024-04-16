// Uncomment this block to pass the first stage
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::str;

use itertools::Itertools;

fn handle_request(request : &str, stream : &mut TcpStream) -> Result<(), &'static str> {
    if request.len() <= 0 {
        return Err("Invalid data provided!");
    }

    let mut request_parts = request.split_whitespace();

    let request_type = request_parts.next().unwrap();
    let request_path = request_parts.next().unwrap();

    if request_path == "/" {
        stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    } else {
        stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
    }


    Ok(())
}

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
                let user_request = std::str::from_utf8_mut(&mut buffer).unwrap();
                handle_request(user_request, &mut _stream).unwrap();
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
