use std::io::{BufRead, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                thread::spawn(move || {
                    handle_connect(tcp_stream);
                });
            }

            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connect(mut tcp_stream: TcpStream) {
    println!("accepted new connection");

    let mut buffer = [0; 50];
    while let Ok(n) = tcp_stream.read(&mut buffer) {
        if n == 0 {
            // Connection was closed
            break;
        }
        let msg = String::from_utf8_lossy(&buffer[..n]);
        println!("read bytes {:?}, string {:?}", &buffer[..n], msg);
        if msg == "*1\r\n$4\r\nping\r\n" {
            // TcpStream::write_all only returns if all provided data is written,
            // write might return if the data is partially written
            let _ = tcp_stream.write_all(b"+PONG\r\n");
        }
    }
}