use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};


pub fn forward_request(tcp_stream: TcpStream, send_payload: &str){

    // connect to the dest server
    let mut stream = TcpStream::connect(tcp_stream).expect(
        "connect to fail destination host"
    );

    let payload = payload.as_bytes();
    // send data to dest
    stream.write_all(payload).
        expect(
            "fail to send data to destination server"
        );




}