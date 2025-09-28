use std::io::{Read, Write};
use std::net::{TcpStream,TcpListener};


pub fn handle_client(mut tcp_stream: TcpStream){
    let mut buffer = [0;4096];
    
    tcp_stream.read((&mut buffer)).expect(
        "fail to read client data"
    );
    
    // convert buffer data to string
    let data = String::from_utf8_lossy(&buffer[..]);  //decode byte -> string
    
    println!("Recived request : {}", data);
    
    let response = "HTTP/1.1 200 OK\r\n\
                         Content-Type: text/plain\r\n\r\n\
                         Hii hello client i am sorting-hat-load-balancer".as_bytes();
    
    tcp_stream.write(response).expect(
        "fail send res data to client"
    );
    
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect(
        "fail to bind address 127.0.0.1:8080"
    );
    
    println!("");
}
