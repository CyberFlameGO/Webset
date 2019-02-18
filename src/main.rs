use std::net::TcpListener;
use std::net:TcpStream;
use std::io::prelude::*;

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

   for _stream in listener.incoming() {
       let stream = _stream.unwrap();
       print!("Connection made...");
   }
}

fn handle_connection(mut stream: TcpStream) {

}
