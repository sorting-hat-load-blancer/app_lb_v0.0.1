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

    let mut buffer = [0;1024];

    match stream.read(&mut buffer){
        Ok(bytes_read) => {
            let response = String::from_utf16_lossy(&buffer[..bytes_read]);
            println!("Received response {}", response);
        }
        Err(e) => {
            eprintln!("fail to read data from server {}",e);
        }
    }

    Ok(());



}