extern crate chrono;

use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::prelude::*;
use std::path::PathBuf;
use std::env;
use std::thread;
use std::sync::mpsc;

use chrono::prelude::*;

/*
The logger will use the ELF system. It will run in a separate thread by using a mpsc
channel, in which other threads will add logs, and this thread will write into the appropiate file.
*/

pub struct Logger {
	dt: DateTime<Local>,
	file: String,
	thread: thread::JoinHandle<()>,
	channel: mpsc::Sender<String>,
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
	pub fn new () -> Logger {
		// Get date and time
		let time: DateTime<Local> = Local::now();
		println!("Time: {:?}", time);

		// Create file
		fs::create_dir_all("logs").unwrap();
		let file = format!("logs/log_{}.log", time.format("%d-%m-%Y_%H-%M-%S"));
		File::create(&file).expect("Could not create file.");

		// Initialize channel
		let (sender, reciever) = mpsc::channel();
		sender.send("# Version: 1.0".to_string()).unwrap();
		sender.send(format!("# Date: {}", time.format("%d-%m-%Y_%H-%M-%S"))).unwrap();
		sender.send("# Fields: To be defined".to_string()).unwrap();

		// Spawn thread
		let file_thread = file.clone();
		let thread = thread::spawn(move || {
			loop {
				let msg = reciever.recv().unwrap();
				let file = File::open(&file_thread).unwrap();
				let mut file = BufWriter::new(file);
				println!("Writing {:?} into {:?}", msg, file_thread);
				file.write_all(&msg.into_bytes()).expect("Could not write into file.");
			}
		});

		Logger { 
			dt: time,
			file: file, 
			thread: thread,
			channel: sender,
		}
	}

	pub fn log (&self, level: LogLevel, msg: &str) {

	}
}