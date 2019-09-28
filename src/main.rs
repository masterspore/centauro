// Module imports
mod http;
mod lib;

// System I/O
use std::fs;
use std::io::prelude::*;
use std::borrow::Cow;

// Network
use std::net::{TcpListener, TcpStream};


fn main() {
	let address = String::from("127.0.0.1:7878");
	println!("Booting server at address: {}", address);

	let listener = TcpListener::bind(&address).unwrap();
	let pool = lib::ThreadPool::new(4);

	println!("Successfully booted server.");

	// listener.incoming() returns an iterator, but instead of being static, it
	// gets a new stream inside every time a new connection is made. 
	// (The iterator has no 'Close' (or something similar), but a 'None'?)
	for stream in listener.incoming() {
		let stream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});
	}
}

/*
Handles the TcpStream provided by the main function. After trying to read the request,
it derives the HttpRequest to the corresponding function, based on the method of the
request.
*/

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
		Cow::Owned(_new) => println!("There was something wrong reading the request"),
	}

	if success_reading_request {
		let http_request = http::parse_http_request(&request).unwrap();

		match http_request.method {
			http::HttpMethod::GET => process_get_request(&http_request, stream),
			_ => (),
		}
	}
}

/*
Takes the client's TcpStream and their request, if it's a GET. The function
returns the file that's being asked for, if it exists in the whitelist.
Otherwise, it returns a 404 for security reasons.
*/

fn process_get_request (request: &http::HttpRequest, mut stream: TcpStream) {
	let mut filename = String::from("/404.html");
	let mut status_line = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");

	match file_in_whitelist(&request.params) {
		Ok(file) => {
			filename = file;
			status_line = String::from("HTTP/1.1 200 OK\r\n\r\n");
		},
		_ => (),
	}

	filename = format!("html{}", filename);
	println!("filename: {:?}", filename);

	let contents = fs::read_to_string(filename).unwrap();
	let response = format!("{}{}", status_line, contents);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}

/*
Checks whether the file that's being asked for in a GET request is accessible.
The whitelist file can be modified by administrators, and '/' returns the
index page.
*/

fn file_in_whitelist (param: &String) -> Result<String, http::HttpError> {
	let whitelist_file = fs::read_to_string("html/_whitelist.txt").unwrap();
	let whitelist: Vec<&str> = whitelist_file.lines().collect();

	println!("whitelist: {:?}", whitelist);

	if param == "/" { return Ok(String::from("/hello.html")); }

	for file in &whitelist {
		if file == param { return Ok(param.to_string()) }
	}

	Err(http::HttpError::new("File not found", param))
}