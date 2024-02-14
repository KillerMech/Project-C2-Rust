/* 
   This is how Rust implements library files.
   This library is a simple thread pool implementation.
*/
use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn  new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Create a channel for the workers to communicate with the ThreadPool.
        let (sender, receiver) = mpsc::channel();
        // Wrap the receiver in an Arc and a Mutex to make it thread safe
        // and allow multiple workers to share the receiver.
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        // Create the workers and store them in the vector.
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // The execute method takes a closure and sends it to the workers.
    // A closure is a function that captures its environment.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    // This method creates a new worker and starts a new thread.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn( move || loop {
            // TODO: Change unwrap to expect and handle the error.
            // lock is called on receiver to acquire the mutex.
            // recv is called after lock to get a job from the receiver.
            // recv blocks until a job becomes available.
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}