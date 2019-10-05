use std::thread;
use std::sync::{mpsc, Arc, Mutex};

// TODO: Try to understand this code and comment how everything works,
// Right now it's starting to look like black magic.

#[derive(Debug)]	
pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>,
}

impl ThreadPool {
	pub fn new(size: usize) -> ThreadPool {
		assert!(size > 0);

		let (sender, reciever) = mpsc::channel();
		let reciever = Arc::new(Mutex::new(reciever));
		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&reciever)));
		}

		ThreadPool {
			workers,
			sender,
		}
	}

	/*
	Executes a thread if we are under maximum load capacity, to prevent DDOS attacks.
	*/

	pub fn execute<F>(&self, f: F) 
		where
			F: FnOnce() + Send + 'static
	{
		let job = Box::new(f);
		self.sender.send(job).unwrap();
	}
}

#[derive(Debug)]
struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>,
}

impl Worker {
	fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let thread = thread::spawn(move || {
			loop {
				let job = reciever.lock().unwrap().recv().unwrap();
				job.call_box();
			}
		});

		Worker {
			id,
			thread,
		}
	}
}

trait FnBox {
	fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
	fn call_box(self: Box<F>) {
		(*self)()
	}
}

type Job = Box<dyn FnBox + Send + 'static>;
