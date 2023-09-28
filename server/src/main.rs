use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8090").expect("Unable to bind");
    for stream_result in listener.incoming() {
        if let Ok(stream) = stream_result {
            thread::spawn(|| {
                handle_stream_connection(stream);
            });
        }
    }
}

// Intensive response time & request processing.
fn handle_stream_connection(mut stream: TcpStream) {
    let request_stream = BufReader::new(&stream);
    let content = request_stream.lines().next().unwrap().unwrap();
    let (status_code, file_path) = match &content[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./success.html"),
        _ => ("HTTP/1.1 404 Not Found", "./not_found.html"),
    };
    let file = fs::read_to_string(file_path).expect("Unable to open file");
    let length = file.len();
    let response = format!("{status_code}\r\nContent-Length: {length}\r\n\r\n{file}");

    // Mimic heavy response time.
    thread::sleep(Duration::from_secs(5));

    // Respond to request.
    stream.write_all(response.as_bytes()).unwrap();
}
