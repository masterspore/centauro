extern crate chrono;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::sync::{mpsc, Mutex, Arc}	;
use std::collections::HashMap;

use chrono::prelude::*;

/*
It will run in a separate thread by using a mpsc channel, in which other threads will add logs, 
and this thread will write into the appropiate file.

USAGE:
First, create the logging channel:
mod log;
use log::*;

let log_ch = log::begin(&config);

Begin returns an Arc+Mutex wrapped sender channel. Send copies of the sender to threads whom
might want to log messages.

To log, you can use the auxiliary function:
fn log<'a> (level: LogLevel, message: String, sender: &'a Arc<Mutex<mpsc::Sender<LogMessage>>>)
*/

pub struct LogMessage {
	pub level: LogLevel,
	pub message: String,
}

pub enum LogLevel {
	DEBUG,
	INFO,
	NOTICE,
	WARN,
	ERROR,
	FATAL,
}

pub fn begin (config: &HashMap<String, String>) -> Arc<Mutex<mpsc::Sender<LogMessage>>> {
	// Get date and time
	let time: DateTime<Local> = Local::now();
	// Create file
	fs::create_dir_all("logs").unwrap();
	let file = format!("logs/log_{}.log", time.format("%d-%m-%Y_%H-%M-%S"));
	let mut f = File::create(&file).expect("Could not create file.");

	// Initialize channel
	let (sender, reciever): (mpsc::Sender<LogMessage>, mpsc::Receiver<LogMessage>) = mpsc::channel();
	let sender = Arc::new(Mutex::new(sender));

	f.write_all(&format!("# Version: 1.0\n# Date: {}\n# Fields: To be defined\n", time.format("%d-%m-%Y_%H-%M-%S")).into_bytes()).unwrap();

	let log_config = parse_log_config(&config);

	// Spawn thread
	thread::spawn(move || {
		loop {
			let time: DateTime<Local> = Local::now();
			let msg = reciever.recv().unwrap();
			match msg.level {
				LogLevel::DEBUG => {
					let text = format!("{}\t{}\t{}", time.format("%d-%m-%Y %H:%M:%S"), "DEBUG", msg.message);
					if log_config.debug.log { f.write_all(&format!("{}\n", text).into_bytes()).unwrap(); }
					if log_config.debug.print { println!("{}", text); }
				},

				LogLevel::INFO => {
					let text = format!("{}\t{}\t{}", time.format("%d-%m-%Y %H:%M:%S"), "INFO", msg.message);
					if log_config.info.log { f.write_all(&format!("{}\n", text).into_bytes()).unwrap(); }
					if log_config.info.print { println!("{}", text); }
				},

				LogLevel::NOTICE => {
					let text = format!("{}\t{}\t{}", time.format("%d-%m-%Y %H:%M:%S"), "NOTICE", msg.message);
					if log_config.notice.log { f.write_all(&format!("{}\n", text).into_bytes()).unwrap(); }
					if log_config.notice.print { println!("{}", text); }
				},

				LogLevel::WARN => {
					let text = format!("{}\t{}\t{}", time.format("%d-%m-%Y %H:%M:%S"), "WARN", msg.message);
					if log_config.warn.log { f.write_all(&format!("{}\n", text).into_bytes()).unwrap(); }
					if log_config.warn.print { println!("{}", text); }
				},

				LogLevel::ERROR => {
					let text = format!("{}\t{}\t{}", time.format("%d-%m-%Y %H:%M:%S"), "ERROR", msg.message);
					if log_config.error.log { f.write_all(&format!("{}\n", text).into_bytes()).unwrap(); }
					if log_config.error.print { println!("{}", text); }
				},

				LogLevel::FATAL => {
					let text = format!("{}\t{}\t{}", time.format("%d-%m-%Y %H:%M:%S"), "FATAL", msg.message);
					if log_config.fatal.log { f.write_all(&format!("{}\n", text).into_bytes()).unwrap(); }
					if log_config.fatal.print { println!("{}", text); }
				},
			};
		}
	});

	sender
}

fn parse_log_config (config: &HashMap<String, String>) -> LogConfig {
	LogConfig {
		debug: kv_to_loginfo(config.get("log_debug").unwrap().to_string(), config.get("print_debug").unwrap().to_string()),
		info: kv_to_loginfo(config.get("log_info").unwrap().to_string(), config.get("print_info").unwrap().to_string()),
		notice: kv_to_loginfo(config.get("log_notice").unwrap().to_string(), config.get("print_notice").unwrap().to_string()),
		warn: kv_to_loginfo(config.get("log_warn").unwrap().to_string(), config.get("print_warn").unwrap().to_string()),
		error: kv_to_loginfo(config.get("log_error").unwrap().to_string(), config.get("print_error").unwrap().to_string()),
		fatal: kv_to_loginfo(config.get("log_fatal").unwrap().to_string(), config.get("print_fatal").unwrap().to_string()),
	}
}

fn kv_to_loginfo (log: String, print: String) -> LogInfo {
	let l = if log == "1".to_string() { true } else { false };
	let p = if print == "1".to_string() { true } else { false };
	LogInfo { print: p , log: l }
}

struct LogConfig {
	debug: LogInfo,
	info: LogInfo,
	notice: LogInfo,
	warn: LogInfo,
	error: LogInfo,
	fatal: LogInfo,
}

struct LogInfo {
	print: bool,
	log: bool,
}

pub fn log<'a> (level: LogLevel, message: String, sender: &'a Arc<Mutex<mpsc::Sender<LogMessage>>>) {
	sender.lock().unwrap().send(LogMessage {
		level,
		message,
	}).unwrap();
}