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

	let request: Vec<&str> = lines[0].split_whitespace().collect();
	let method = get_method_from_str(request[0]);

	let mut host = Host {ip: " ".to_string(), port: " ".to_string()};
	let mut agent = " ".to_string();

	// FIX THIS
	let params = if request.len() > 2 {
		request[1].to_string()
	} else {
		" ".to_string()
	};
	
	let version = if request.len() > 3 {
		request[2].to_string()
	} else {
		" ".to_string()
	};

	// In case the other parameters are not ordered, we match them
	// Also, pattern matching allows for easier code upgrades
	for line in &lines { 
		let line_vec: Vec<&str> = line.split_whitespace().collect();
		if line_vec.len() > 0 {
			match line_vec[0] {
				"Host:" => {
					let full: Vec<&str> = line_vec[1].split(":").collect();
					if full.len() >= 2 { host = Host {ip: full[0].to_string(), port: full[1].to_string()}; }
				},
				"User-Agent:" => agent = line_vec[1].to_string(),
				_ => (),
			}
		}
	}

	Ok(HttpRequest {
		method,
		params,
		version,
		agent,
		host,
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
	pub host: Host,	
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