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
    sender: Option<mpsc::Sender<Job>>,
}

// The Job type is a type alias for a trait object that holds the type of 
// closure that execute will receive. FnOnce indicates that the closure can be
// called once and Send indicates that the closure can be sent from one thread 
// to another.
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

        ThreadPool { workers, sender: Some(sender) }
    }

    // The execute method takes a closure and sends it to the workers.
    // A closure is a function that captures its environment.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop the sender first to signal to the workers that they should shut down.
        drop(self.sender.take());

        // Join each worker's thread to make sure they finish their work.
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // If the worker has a thread, take it and join it.
            if let Some(thread) = worker.thread.take() {
                // The join method blocks the current thread until the worker's thread has finished.
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // This method creates a new worker and starts a new thread.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn( move || loop {
            // TODO: Change unwrap to expect and handle the error.
            // lock is called on receiver to acquire the mutex.
            // recv is called after lock to get a job from the receiver.
            // recv blocks until a job becomes available.
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                },
                Err(_) => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker { 
            id, 
            thread: Some(thread), 
        }
    }
}
