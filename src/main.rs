use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Should be able to bind");

    for stream in listener.incoming() {
        handle_connection(stream);
    }
}

fn handle_connection(stream: std::io::Result<TcpStream>) {
    match stream {
        Ok(mut stream) => {
            stream
                .write_all(b"HTTP/1.1 200 OK\r\n\r\n")
                .expect("All bytes should be written to the TCP stream");
        }
        Err(e) => {
            println!("error: {e}");
        }
    }
}
