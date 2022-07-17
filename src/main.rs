use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::net::TcpListener;

fn handle_heartbeat() {
  //let mut buffer = [0; 1024];
  let buffer = "Testing 123";
  loop {
    println!("Attempting connection");
    let mut stream = TcpStream::connect("192.168.21.105:30003").unwrap();
    stream.write(buffer.as_bytes()).unwrap();
    stream.flush().unwrap();
    thread::sleep(Duration::from_secs(5));
  }
}

fn handle_incoming() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = "123 Testing";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn main() {
  
  let workthread = thread::spawn( || {
    handle_heartbeat();
  });

  let incomingthread = thread::spawn( || {
    handle_incoming();
  });

  workthread.join().unwrap();
  incomingthread.join().unwrap();
}
