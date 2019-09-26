/*pub #[derive(Debug)]	
struct ThreadPool;

impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		ThreadPool
	}
}*/

pub mod utils {
	pub fn read_request_file(request: &str) -> &'a str {
		search_case_insensitive("GET", request)
	}

	pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> &'a str {
		let query = query.to_lowercase();
		let mut results = Vec::new();

		for line in contents.lines() {
			if line.to_lowercase().contains(&query) {
				return line
			}
		}

		return "none"
	}
}