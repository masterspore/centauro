pub mod Http {
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

	pub fn parse_http_request(request: &String) -> Result<HttpRequest, HttpError> {
		let lines: Vec<&str> = request.lines().collect(); // This returns a vector with each line unwrapped from Option

		let request: Vec<&str> = lines[0].split_whitespace().collect();
		println!("Request: {:?}", request);

		let method = get_method_from_str(request[0]);
		println!("Method: {:?}", method);

		Ok(HttpRequest {
			method: HttpMethod::GET,
			params: request[1].to_string(),
			version: " ".to_string(),
			agent: " ".to_string(),
			host: Host {ip: " ".to_string(), port: " ".to_string()},
		})
	}

	fn get_method_from_str(&method: str) -> HttpMethod {
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
		method: HttpMethod,
		params: String,
		version: String,
		agent: String,	
		host: Host,	
	}

	#[derive(Debug)]
	pub struct Host {
		ip: String,
		port: String,
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
}