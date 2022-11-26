use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}};


fn handle_connection(mut stream: TcpStream){
    println!("Connection made!");

    let buf_reader = BufReader::new(&mut stream);

    let request: Vec<_> = buf_reader
        .lines()
        .map(|r| r.unwrap())
        .take_while(|l| !l.is_empty())
        .collect();

    println!("Request: {:#?}", request);

    let response = "Hello!";
    stream.write_all(response.as_bytes()).unwrap();
    println!("Response sent to {:#?}!", stream.peer_addr());
}


pub fn accept_connections(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}