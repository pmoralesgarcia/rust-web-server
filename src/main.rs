use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    /* port 8447 for TcpListener */
    const HOST : &str ="127.0.0.1";
    const PORT : &str ="8477";

    let end_point : String = HOST.to_owned() + ":" +  PORT;

    let listener = TcpListener::bind(end_point).unwrap();

    println!("Web server is listening at port {}",PORT);

    /* connecting to any incoming connections */
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        
        // Call function to process any incoming connections
        handle_connection(_stream);

    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();    let response_contents = fs::read_to_string("index.html").unwrap();    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_contents.len(),
        response_contents
    );    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
