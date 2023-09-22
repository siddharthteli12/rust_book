use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(client_info) => {
                handle_connection(client_info);
            }
            Err(e) => println!("Connection falied due to - {:}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|result| !result.is_empty())
        .collect();
    let file = fs::read_to_string("./response.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", file);
    stream.write_all(response.as_bytes()).unwrap();
}
