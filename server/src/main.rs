use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(client_info) => {
                println!("Connect with new client - {:?}", client_info);
            }
            Err(e) => println!("Connection falied due to - {:}", e),
        }
    }
}
