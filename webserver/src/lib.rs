use std::{fmt, sync::{mpsc, Arc, Mutex}, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<Arc<Mutex<std::sync::mpsc::Receiver<Job>>>>,
}

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    thread_limit: usize,
    sender: mpsc::Sender<Job>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {

            let job = rx.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job;");

            job();
        }); // this will panic if resources aren available, try
                                           // thread::Builder in a flup
        Worker { id, thread }
    }
}

impl ThreadPool {
    /// Create and manage a ThreadPool
    ///
    ///
    ///
    /// Number of threads is capped to num_threads
    ///
    /// # Panics
    ///
    /// If the number of threads is zero

    // pub fn new(num_threads: usize) -> ThreadPool{
    //     assert!(num_threads > 0);
    //     ThreadPool{
    //         thread_limit: num_threads,
    //         workers: Vec::with_capacity(num_threads)
    //     }
    // }

    pub fn build(num_threads: usize) -> Result<ThreadPool, ThreadError> {
        if num_threads == 0 {
            // PoolCreationError("Number of threads in pool must be greater than zero")

            Err(ThreadError {
                error_code: 0,
                error_message: String::from("0 is an invalid number of threads for a thread pool."),
            })
        } else {
            let (tx, rx) = mpsc::channel();
            let rx = Arc::new(Mutex::new(rx));
            let mut workers = Vec::with_capacity(num_threads);
                
            for id in 0..num_threads {
                workers.push(Worker::new(id, rx.clone()));
            }

            Ok(ThreadPool {
                workers,
                thread_limit: num_threads,
                sender: tx
            })
        }
    }
    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(func);
        self.sender.send(job).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct ThreadError {
    error_code: i64,
    error_message: String,
}

impl fmt::Display for ThreadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "Thread failed with code {} and message {}",
            self.error_code, self.error_message
        )
    }
}
