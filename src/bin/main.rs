use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

use std::thread;
use std::time::Duration;

use std::sync::Arc;
use std::sync::Mutex;


extern crate rust_multithread_with_pool;
use rust_multithread_with_pool::ThreadPool;

// curl -v 127.0.0.1:8080
// as client, which will display the response message

fn main() {
	println!("\nTest in another terminal with command :\n\n curl -v http://localhost:8080\n\n");

	let pool = ThreadPool::new(4);

	let mut counter = 0;

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	for stream in listener.incoming() {
		if counter == 4 {
			println!("Shutting down.");
			break;
		}

		counter += 1;

		let stream = stream.unwrap();

		println!("Connection established!");
		pool.execute(|| {
			handle_connection(stream);
		});

	}
}

fn handle_connection(mut stream: TcpStream) {

	let mut buffer = [0; 512];
	stream.read(&mut buffer).unwrap();

	// simulate sleep
	let   get = b"GET / HTTP/1.1\r\n";
	let sleep = b"GET /sleep HTTP/1.1\r\n";

	let (status_line, filename) = if buffer.starts_with(get) {
		("HTTP/1.1 200 OK\r\n\r\n", "hello.html")

	} else if buffer.starts_with(sleep) {
		thread::sleep(Duration::from_secs(2));
		("HTTP/1.1 200 OK\r\n\r\n", "hello.html")

	} else {
		("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
	};

	// ----
	let mut file = File::open(filename).unwrap();

	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();

	//println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

	let response = format!("{}\r\n\r\n{}", status_line, contents);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();

}
