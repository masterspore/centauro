// System IO
use std::fs;
use std::io::prelude::*;
use std::sync::{mpsc, Mutex, Arc};

// Crate import (from main.rs)
use crate::log;
use crate::log::*;
use crate::http;

// Network
use std::net::TcpStream;

// GET & HEAD parser
pub fn process_get_request (request: &http::HttpRequest, mut stream: TcpStream, is_head: bool, logger: Arc<Mutex<mpsc::Sender<log::LogMessage>>>) {
	let mut file = fs::read_to_string(String::from("public/404.html")).unwrap();
	let mut status_line = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");

	let request_extension: Vec<&str> = request.params.split(".").collect(); // This is to decide wheter we want a folder or specific file
	
	log(LogLevel::DEBUG, format!("GET {} for addr {:?}", request.params, stream.peer_addr().unwrap().ip()), &logger);

	if request.params == "/".to_string() {
		log(LogLevel::DEBUG, format!("Returning main index: {:?}", request.params), &logger);
		match fs::read_to_string(String::from("public/index.html")) {
			Ok(f) => {
				file = f;
			},
			Err(e) => log(LogLevel::WARN, format!("Main index file was not found. Error: {}", e), &logger),
		}
	} else if request_extension.len() <= 1 { // Returns index files for non-main folders (e.g: izar.cc/jan) 
		log(LogLevel::DEBUG, format!("Returning index: {}", request.params), &logger);
		match fs::read_to_string(format!("public{}/index.html", request.params)) {
			Ok(f) => {
				file = f;
			},
			Err(e) => log(LogLevel::WARN, format!("Index file was not found. Error: {}", e), &logger),
		}
	} else {
		log(LogLevel::DEBUG, format!("Returning file: {}", request.params), &logger);
		match fs::read_to_string(format!("public{}", request.params)) {
			Ok(f) => {
				status_line = String::from("HTTP/1.1 200 OK\r\n\r\n");
				file = f;
			},
			Err(e) => log(LogLevel::WARN, format!("File {} was not found. Error: {}", request.params, e), &logger),
		} 
	}

	let response = if is_head { status_line } else { format!("{}{}", status_line, file) };
	log(LogLevel::INFO, format!("Sending file {}.", request.params), &logger);

	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}

// POST parser
/*
fn process_post_request (request: &http::HttpRequest, mut stream: TcpStream) {
	let mut file = fs::read_to_string(String::from("public/404.html")).unwrap(); 
	let mut status_line = String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");

	let payload: Vec<&str> = request.payload.split("&").collect();

	match fs::read_to_string(format!("public{}", request.params)) {
		Ok(f) => {
			status_line = String::from("HTTP/1.1 201 OK\r\n\r\n"); // 201 -> created
			file = f;
		},
		Err(e) => (),
	} 

	//stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}
*/