use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;

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

    let mut file = File::open("test.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n", contents);
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}   
