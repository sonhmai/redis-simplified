use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Command;
use std::sync::{Arc, Barrier, mpsc};
use std::thread;
use std::time::Duration;

fn start_server_process() -> std::process::Child {
    let server = Command::new("cargo")
        .arg("run")
        .spawn()
        .expect("failed to start server");
    thread::sleep(Duration::from_secs(1));
    server
}

#[test]
fn test_ping_pong() {
    let mut server = start_server_process();

    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    let mut buffer = [0; 512];

    // send 1 ping, expect 1 pong response
    stream.write_all(b"*1\r\n$4\r\nping\r\n").unwrap();
    stream.read(&mut buffer).unwrap();
    assert_eq!(&buffer[..7], b"+PONG\r\n");

    // send 2nd ping, expect pong back
    stream.write_all(b"*1\r\n$4\r\nping\r\n").unwrap();
    stream.read(&mut buffer).unwrap();
    assert_eq!(&buffer[..7], b"+PONG\r\n");


    server.kill().expect("failed to kill server");
}

#[test]
fn test_concurrent_clients_should_run_within_timeout() {
    let mut server = start_server_process();

    let barrier = Arc::new(Barrier::new(3)); // synchronize concurrent clients

    let test_client = |id: i32, barrier: Arc<Barrier>| {
        barrier.wait(); // Wait here to sync all clients before doing below

        let mut client = TcpStream::connect("127.0.0.1:6379").unwrap();

        client.set_read_timeout(Some(Duration::from_secs(1))).expect("Failed to set read timeout");
        let mut buffer = [0; 512];

        println!("Client {} sending message", id);
        client.write_all(b"*1\r\n$4\r\nping\r\n").unwrap();


        client.read(&mut buffer).unwrap();
        assert_eq!(&buffer[..7], b"+PONG\r\n");
        println!("Client {} received response", id);
    };

    let barrier_clone = Arc::clone(&barrier);
    let client_one = thread::spawn(move || test_client(1, barrier_clone));

    let barrier_clone = Arc::clone(&barrier);
    let client_two = thread::spawn(move || test_client(2, barrier_clone));

    let barrier_clone = Arc::clone(&barrier);
    let client_three = thread::spawn(move || test_client(3, barrier_clone));

    client_one.join().unwrap();
    client_two.join().unwrap();
    client_three.join().unwrap();

    server.kill().expect("failed to kill server");
}

#[test]
fn test_echo() {
    let mut server = start_server_process();

    let mut stream = TcpStream::connect("127.0.0.1:6379").unwrap();
    let mut buffer = [0; 512];

    // *2 -> 2 strings
    // $4 -> next string len 4
    // echo -> string 1
    // $3 -> next string len 3
    // hey -> string 2
    stream.write_all(b"*2\r\n$4\r\necho\r\n$3\r\nhey\r\n").unwrap();
    let bytes_read = stream.read(&mut buffer).unwrap();
    assert_eq!(&buffer[..bytes_read], b"$3\r\nhey\r\n");

    server.kill().expect("failed to kill server");
}