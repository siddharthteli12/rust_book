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
    let request: String = buf_reader.lines().next().unwrap().unwrap();

    let (status, filename) = if request == "GET / HTTP/1.1".to_string() {
        ("HTTP/1.1 200 OK", "./response.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let content_length = contents.len();
    let response = format!("{status}\r\nContent-Length: {content_length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
