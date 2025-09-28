use std::io::{Read, Write};
use std::net::{TcpStream,TcpListener};


pub fn handle_client(mut tcp_stream: TcpStream){
    let mut buffer = [0;4096];
    
    tcp_stream.read((&mut buffer)).expect(
        "fail to read client data"
    );
    
}

fn main() {
    println!("Hello, world!");
}
