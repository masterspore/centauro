use std::io;
use std::io::Read;
use std::fs::File;

use std::collections::HashMap;

/*
load_config reads a config file ('config.ini') and returns a hash map with all
the read variables. It ignores lines that start with '#'.
*/

pub fn load_config(config: &str) -> Result<HashMap<String, String>, io::Error> {
	let mut config_map = HashMap::new();

	println!("Loading file: {}", config);
	let mut file = match File::open(config) {
		Ok(f) => f,
		Err(e) => {
			println!("There was something wrong reading the file, creating '{}' and crashing.", config);
			println!("Please fill out the config file created.");
			File::create(config).expect("Could not create file.");
			return Err(e);
		},
	};

	let mut contents = String::new();
	file.read_to_string(&mut contents);
	let contents: Vec<&str> = contents.lines().collect();

	for line in &contents {
		let first_char = line.chars().next();
		match first_char {
			Some(c) => {
				if c != '#' {
					let options: Vec<&str> = line.split("=").collect();
					config_map.insert(options[0].to_string(), options[1].to_string());
				}
			},
			_ => (), // Empty line
		}
	}

	Ok(config_map)
}