// Module imports
mod http;
mod lib;

// System I/O
use std::fs;
use std::io::prelude::*;
use std::borrow::Cow;

// Network
use std::net::{TcpListener, TcpStream};

// Threading
use std::thread;
use std::time::Duration;

// Error handling
use std::error::Error;
use std::fmt;

//use simple_server::ThreadPool;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	//let pool = ThreadPool::new(4);

	for stream in listener.incoming() {
		let stream = stream.unwrap();

		//pool.execute(|| {
			handle_connection(stream);
		//});
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buffer = [0; 512];
	stream.read(&mut buffer).unwrap();

	let mut request = String::from(""); // Read (from README) 'the secret life of cows'
	let mut success_reading_request = false;

	match String::from_utf8_lossy(&buffer[..]) {
		Cow::Borrowed(req) => {
			request.push_str(req);
			success_reading_request = true;
		},
		Cow::Owned(new) => println!("There was something wrong reading the request"),
	}

	if success_reading_request {
		println!("Raw request: {}", &request);

		let get = b"GET / HTTP/1.1\r\n";
		let sleep = b"GET /sleep HTTP/1.1\r\n";

		println!("We read: {:?}", http::Http::parse_http_request(&request));

		let (status_line, filename) = if buffer.starts_with(get) {
	        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
	    } else if buffer.starts_with(sleep) {
	    	thread::sleep(Duration::from_secs(5));
	    	("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
	    } else {
	        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "html/404.html")
	    };

	    let contents = fs::read_to_string(filename).unwrap();
	    let response = format!("{}{}", status_line, contents);

	    stream.write(response.as_bytes()).unwrap();
	}

    stream.flush().unwrap();
}