use std::{fmt, sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Builds a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Returns
    /// A `Result<ThreadPool, PoolCreationError` depending on `size`.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> { 
        match size > 0 {
            true => {
                let (sender, receiver) = mpsc::channel();
                let receiver = Arc::new(Mutex::new(receiver));

                let mut workers = Vec::with_capacity(size);

                for id in 0..size {
                    workers.push(Worker::new(id, Arc::clone(&receiver)));
                }

                Ok(ThreadPool { workers, sender: Some(sender) })
            },
            _ => Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f:F) where F:FnOnce() + Send + 'static, {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
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
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = reciever.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                }
                Err(_) => {
                    print!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}

#[derive(Debug, Clone)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid pool size")
    }
}
