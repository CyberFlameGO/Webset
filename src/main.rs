use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

   for _stream in listener.incoming() {
       let stream = _stream.unwrap();
       print!("Connection made...");
   }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Requested {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}   
