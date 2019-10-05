extern crate chrono;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;

use chrono::prelude::*;

/*
It will run in a separate thread by using a mpsc channel, in which other threads will add logs, 
and this thread will write into the appropiate file.
*/

pub struct Logger {
	pub sender: mpsc::Sender<String>,
	debug: LogInfo,
	info: LogInfo,
	notice: LogInfo,
	warn: LogInfo,
	error: LogInfo,
	fatal: LogInfo,
}

pub struct LogInfo {
	print: bool,
	log: bool,
}

pub enum LogLevel {
	DEBUG,
	INFO,
	NOTICE,
	WARN,
	ERROR,
	FATAL,
}

impl Logger {
	pub fn new (config: &HashMap<String, String>) -> Logger {
		// Get date and time
		let time: DateTime<Local> = Local::now();
		// Create file
		fs::create_dir_all("logs").unwrap();
		let file = format!("logs/log_{}.log", time.format("%d-%m-%Y_%H-%M-%S"));
		let mut f = File::create(&file).expect("Could not create file.");

		// Initialize channel
		let (sender, reciever) = mpsc::channel();
		sender.send("# Version: 1.0\n".to_string()).unwrap();
		sender.send(format!("# Date: {}\n", time.format("%d-%m-%Y_%H-%M-%S"))).unwrap();
		sender.send("# Fields: To be defined\n".to_string()).unwrap();

		// Spawn thread
		thread::spawn(move || {
			loop {
				let msg = reciever.recv().unwrap();
				f.write_all(&msg.into_bytes()).expect("Could not write into file.");
			}
		});

		// Parse configuration
		let debug = kv_to_loginfo(config.get("log_debug").unwrap().to_string(), config.get("print_debug").unwrap().to_string());
		let info = kv_to_loginfo(config.get("log_info").unwrap().to_string(), config.get("print_info").unwrap().to_string());
		let notice = kv_to_loginfo(config.get("log_notice").unwrap().to_string().to_string(), config.get("print_notice").unwrap().to_string());
		let warn = kv_to_loginfo(config.get("log_warn").unwrap().to_string().to_string().to_string(), config.get("print_warn").unwrap().to_string().to_string());
		let error = kv_to_loginfo(config.get("log_error").unwrap().to_string(), config.get("print_error").unwrap().to_string());
		let fatal = kv_to_loginfo(config.get("log_fatal").unwrap().to_string(), config.get("print_fatal").unwrap().to_string());

		Logger { 
			sender,
			debug,
			info,
			notice,
			warn,
			error,
			fatal,
		}
	}

	pub fn log (&self, level: LogLevel, msg: &str) {
		let time: DateTime<Local> = Local::now();
		let time = time.format("%d-%m-%Y %H:%M:%S");
		match level {
			LogLevel::DEBUG => {
				if self.debug.log { self.sender.send(format!("{}\t{}\t{}\n", time, "DEBUG", msg.to_string())).unwrap(); }
				if self.debug.print { println!("{}", format!("{}\t{}\t{}", time, "DEBUG", msg.to_string())); }
			},

			LogLevel::INFO => {
				if self.info.log { self.sender.send(format!("{}\t{}\t{}\n", time, "INFO", msg.to_string())).unwrap(); }
				if self.info.print { println!("{}", format!("{}\t{}\t{}", time, "INFO", msg.to_string())); }
			},

			LogLevel::NOTICE => {
				if self.notice.log { self.sender.send(format!("{}\t{}\t{}\n", time, "NOTICE", msg.to_string())).unwrap(); }
				if self.notice.print { println!("{}", format!("{}\t{}\t{}", time, "NOTICE", msg.to_string())); }
			},

			LogLevel::WARN => {
				if self.warn.log { self.sender.send(format!("{}\t{}\t{}\n", time, "WARN", msg.to_string())).unwrap(); }
				if self.warn.print { println!("{}", format!("{}\t{}\t{}", time, "WARN", msg.to_string())); }
			},

			LogLevel::ERROR => {
				if self.error.log { self.sender.send(format!("{}\t{}\t{}\n", time, "ERROR", msg.to_string())).unwrap(); }
				if self.error.print { println!("{}", format!("{}\t{}\t{}", time, "ERROR", msg.to_string())); }
			},

			LogLevel::FATAL => {
				if self.fatal.log { self.sender.send(format!("{}\t{}\t{}\n", time, "FATAL", msg.to_string())).unwrap(); }
				if self.fatal.print { println!("{}", format!("{}\t{}\t{}", time, "FATAL", msg.to_string())); }
			},
		}
	}
}

fn kv_to_loginfo (log: String, print: String) -> LogInfo {
	let l = if log == "1".to_string() { true } else { false };
	let p = if print == "1".to_string() { true } else { false };
	LogInfo { print: p , log: l }
}