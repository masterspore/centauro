use std::error::Error;
use std::fmt;

/* The HTTP request sent by web browsers goes as follows:
Request: METHOD PARAMS VERSION
Host: ip:port
User-Agent: --
Accept: --
Accept-Language: --
Accept-Enconding: --
Connection: --
Upgrade-Insecure-Requests: --
*/

/* 
Returns an HttpRequest with the parsed data provided by the client.
*/ 

pub fn parse_http_request(request: &String) -> Result<HttpRequest, HttpError> {
	let lines: Vec<&str> = request.lines().collect(); // This returns a vector with each line unwrapped from Option

	// Parsing the first line is different from the others
	let request: Vec<&str> = lines[0].split_whitespace().collect();

	let method = get_method_from_str(request[0]);
	let params = if request.len() > 2 { request[1].to_string() } else { " ".to_string() };
	let version = if request.len() > 3 { request[2].to_string() } else { " ".to_string() };

	let mut host = Host {ip: " ".to_string(), port: " ".to_string()};
	let mut agent = " ".to_string();
	let mut content_type = " ".to_string();
	let mut content_length = 0;
	let mut accept = " ".to_string();
	let mut accept_language = " ".to_string();
	let mut accept_encoding = " ".to_string();
	let mut connection = " ".to_string();

	// In case the other parameters are not ordered, we match them
	// Also, pattern matching allows for easier code upgrades
	for line in &lines { 
		let line_vec: Vec<&str> = line.split_whitespace().collect();
		println!("Request line: {:?}, length = {:?}", line_vec, line_vec.len());
		if line_vec.len() > 1 {
			match line_vec[0] {
				"Host:" => {
					println!("{:?}", line_vec[1]);
					let full: Vec<&str> = line_vec[1].split(":").collect();
					if full.len() >= 2 { host = Host {ip: full[0].to_string(), port: full[1].to_string()}; }
					else { host = Host {ip: line_vec[1].to_string(), port: "80".to_string()}; }
				},
				"User-Agent:" => agent = line_vec[1].to_string(),
				"Content-Type:" => content_type = line_vec[1].to_string(),
				"Content-Length:" => content_length = line_vec[1].to_string().parse().unwrap(),
				"Accept:" => accept = line_vec[1].to_string(),
				"Accept-Language:" => accept_language = line_vec[1].to_string(),
				"Accept-Encoding:" => accept_encoding = line_vec[1].to_string(),
				"Connection" => connection = line_vec[1].to_string(),
				_ => (),
			}
		}
	}

	let payload = lines[lines.len()-1].to_string();

	Ok(HttpRequest {
		method,
		params,
		version,
		agent,
		accept,
		accept_language,
		accept_encoding,
		host,
		connection,
		content_type,
		content_length,
		payload,
	})
}

fn get_method_from_str(method: &str) -> HttpMethod {
	match method {
		"GET" => return HttpMethod::GET,
		"HEAD" => return HttpMethod::HEAD,
		"POST" => return HttpMethod::POST,
		"PUT" => return HttpMethod::PUT,
		"DELETE" => return HttpMethod::DELETE,
		"CONNECT" => return HttpMethod::CONNECT,
		"OPTIONS" => return HttpMethod::OPTIONS,
		"TRACE" => return HttpMethod::TRACE,
		"PATCH" => return HttpMethod::PATCH,
		_ => return HttpMethod::NONE,
	}
}

#[derive(Debug)]
pub struct HttpRequest {
	pub method: HttpMethod,
	pub params: String,
	pub version: String,
	pub agent: String,
	pub accept: String,
	pub accept_language: String,
	pub accept_encoding: String,
	pub host: Host,	
	pub connection: String,
	pub content_type: String,
	pub content_length: usize,
	pub payload: String,
}

#[derive(Debug)]
pub struct Host {
	pub ip: String,
	pub port: String,
}

#[derive(Debug)]
pub enum HttpMethod {
	GET,
	HEAD,
	POST,
	PUT,
	DELETE,
	CONNECT,
	OPTIONS,
	TRACE,
	PATCH,
	NONE,
}

// Errors must implement:
//	  - Debug
//    - Display
//    - Error

#[derive(Debug)]
pub struct HttpError {
	details: String,
	request: String,
}

impl HttpError {
	pub fn new(msg: &str, req: &String) -> HttpError {
		HttpError {
			details: msg.to_string(),
			request: req.to_string(),
		}
	}
}

impl fmt::Display for HttpError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "details: {} \nrequest: {}", self.details, self.request)
	}
}

impl Error for HttpError {
	fn description(&self) -> &str {
		&self.details
	}
}