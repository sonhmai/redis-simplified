use std::io::{BufRead, Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut tcp_stream) => {
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
                        let _ = tcp_stream.write(b"+PONG\r\n");
                    }
                }
            }

            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
