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
			_ => (),
		}
	}
}

/*
Takes the client's TcpStream and their request, if it's a GET. The function
returns the file that's being asked for, if it exists in the whitelist.
Otherwise, it returns a 404 for security reasons.
*/

fn process_get_request (request: &http::HttpRequest, mut stream: TcpStream, is_head: bool) {
	let mut filename = String::from("/404.html");
	let mut status_line = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");

	match file_in_public_folder(&request.params) {
		Ok(file) => {
			filename = file;
			status_line = String::from("HTTP/1.1 200 OK\r\n\r\n");
		},
		_ => (),
	}

	filename = format!("public{}", filename);
	println!("Returning file: {:?}", filename);

	let contents = fs::read_to_string(filename).unwrap();
	let response = if is_head { status_line } else { format!("{}{}", status_line, contents) };

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}

/*
Checks whether the file is accessible by everyone
*/

fn file_in_public_folder (param: &String) -> Result<String, http::HttpError> {
	let files = fs::read_dir("public").unwrap();

	if param == "/" { return Ok("/index.html".to_string()); }

    for file in files {
    	match file {
    		Ok(f) => {
    			if f.file_name().to_str() == Some(&param[1..]) { return Ok(param.to_string()); } // The [1..] is to exclude the '/'
    		},
    		Err(e) => (),
    	}
    }

    Err(http::HttpError::new("File not found", param))
}

/*
Checks whether the file that's being asked for in a GET request is accessible.
The whitelist file can be modified by administrators, and '/' returns the
index page.
*/

fn file_in_whitelist (param: &String) -> Result<String, http::HttpError> {
	let whitelist_file = fs::read_to_string("public/_whitelist.txt").unwrap();
	let whitelist: Vec<&str> = whitelist_file.lines().collect();

	for file in &whitelist {
		if file == param { return Ok(param.to_string()) }
	}

	Err(http::HttpError::new("File not found", param))
}