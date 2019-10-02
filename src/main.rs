// Module imports
mod http;
mod lib;
mod config;
mod logger;

// System I/O
use std::fs;
use std::io::prelude::*;
use std::borrow::Cow;

// Network
use std::net::{TcpListener, TcpStream};

fn main() {
	println!("Booting server...");

	let config_file = config::load_config("config.ini").unwrap();
	println!("Config file: {:?}", config_file);

	let address = format!("{}:{}", config_file.get("ip_address").unwrap(), config_file.get("port").unwrap());
	let listener = TcpListener::bind(&address).unwrap();
	println!("Address to be used: {}", address);

	let tp_size: usize = config_file.get("thread_pool_size").unwrap().parse().unwrap();
	let pool = lib::ThreadPool::new(tp_size);
	println!("Thread pool of size {} created.", tp_size);

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
		//println!("{:?}", http_request);

		match http_request.method {
			http::HttpMethod::GET => process_get_request(&http_request, stream, false),
			http::HttpMethod::HEAD => process_get_request(&http_request, stream, true),
			//http::HttpMethod::POST => process_post_request(&http_request, stream),
			_ => (),
		}
	}
}

/*
Takes the client's TcpStream and their request, if it's a GET. The function
returns the file that's being asked for, if it exists.
Otherwise, it returns a 404.
*/

fn process_get_request (request: &http::HttpRequest, mut stream: TcpStream, is_head: bool) {
	let mut file = fs::read_to_string(String::from("public/404.html")).unwrap();
	let mut status_line = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");

	if request.params == "/".to_string() {
		file = fs::read_to_string(String::from("public/index.html")).unwrap();
	} else {
		match fs::read_to_string(format!("public{}", request.params)) {
			Ok(f) => {
				status_line = String::from("HTTP/1.1 200 OK\r\n\r\n");
				file = f;
			},
			Err(e) => (),
		} 
	}

	let response = if is_head { status_line } else { format!("{}{}", status_line, file) };

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}

/*
Takes a client's TCP stream and the POST request, then parses it.
*/

/*
fn process_post_request (request: &http::HttpRequest, mut stream: TcpStream) {
	let mut file = fs::read_to_string(String::from("public/404.html")).unwrap(); 
	let mut status_line = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");

	let payload: Vec<&str> = request.payload.split("&").collect();

	match fs::read_to_string(format!("public{}", request.params)) {
		Ok(f) => {
			status_line = String::from("HTTP/1.1 200 OK\r\n\r\n");
			file = f;
		},
		Err(e) => (),
	} 

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}
*/