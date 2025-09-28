use std::io::{Read, Write};
use std::net::{TcpStream,TcpListener};


fn handle_client(mut tcp_stream: TcpStream){
    let mut buffer = [0;4096];
    
    tcp_stream.read(&mut buffer).expect(
        "fail to read client data"
    );
    
    // convert buffer data to string
    let data = String::from_utf8_lossy(&buffer[..]);  //decode byte -> string
    
    println!("Recived request : {}", data);
    
    let response = "HTTP/1.1 200 OK\r\n\
                         Content-Type: text/plain\r\n\r\n\
                         Hii hello client i am sorting-hat-loadBalancer".as_bytes();
    
    tcp_stream.write(response).expect(
        "fail send res data to client"
    );
    
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect(
        "fail to bind address 127.0.0.1:8080"
    );
    
    println!("Sorting-Hat-LoadBalancer run on 127.0.0.1:8080");
    
    loop {
        match listener.accept() { 
            Ok((tcp_stream, address)) => {
                println!("Accept conn : {} {:?}", address, tcp_stream);

                // create a new separate thread for each connection

                std::thread::spawn(|| {
                    handle_client(tcp_stream);
                });
            }

            Err(e) =>{
                eprintln!("error accepting conn : {}" , e);
            }
        }
        
    }
}
