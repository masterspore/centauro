// Module imports
mod http;
mod lib;
mod config;
mod log;
mod parser;

// System I/O
use std::io::prelude::*;
use std::borrow::Cow;
use std::sync::{mpsc, Mutex, Arc};

use log::*;

// Network
use std::net::{TcpListener, TcpStream};

fn main() {
	let config_file = config::load_config("config.ini").unwrap();
	let log_ch = log::begin(&config_file);
	log(LogLevel::INFO, "Booting server...".to_string(), &log_ch);

	let address = format!("{}:{}", config_file.get("ip_address").unwrap(), config_file.get("port").unwrap());
	let listener = TcpListener::bind(&address).unwrap();
	log(LogLevel::INFO, format!("Address to be used: {}", address), &log_ch);

	let tp_size: usize = config_file.get("thread_pool_size").unwrap().parse().unwrap();
	let pool = lib::ThreadPool::new(tp_size);
	log(LogLevel::DEBUG, format!("Thread pool of size {} created.", tp_size), &log_ch);

	log(LogLevel::INFO, "Successfully booted server.".to_string(), &log_ch);

	// listener.incoming() returns an iterator, but instead of being static, it
	// gets a new stream inside every time a new connection is made. 
	// (The iterator has no 'Close' (or something similar), but a 'None'?)

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		let _log = log_ch.clone();

		pool.execute(move || {
			handle_connection(stream, _log);
		});
	}
}

/*
Handles the TcpStream provided by the main function. After trying to read the request,
it derives the HttpRequest to the corresponding function, based on the method of the
request.
*/

fn handle_connection(mut stream: TcpStream, logger: Arc<Mutex<mpsc::Sender<log::LogMessage>>>) {
	let mut buffer = [0; 512];
	let mut request = String::from(""); // Read (from README) 'the secret life of cows'
	let mut success_reading_request = false;

	stream.read(&mut buffer).unwrap();
	log(LogLevel::INFO, format!("Incoming connection: {:?}", stream.peer_addr().unwrap().ip()), &logger);

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
			http::HttpMethod::GET => parser::process_get_request(&http_request, stream, false, logger),
			http::HttpMethod::HEAD => parser::process_get_request(&http_request, stream, true, logger),
			//http::HttpMethod::POST => parser::process_post_request(&http_request, stream),
			_ => (),
		}
	}
}