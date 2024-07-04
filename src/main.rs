use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Should be able to bind");

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {e}");
            }
        }
    }
}
