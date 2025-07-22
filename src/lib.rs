use std::thread;
use std::sync::mpsc;

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSize,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

struct Job;

impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker {
            id,
            thread,
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::ZeroSize)
        }

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver));
        }

        Ok(ThreadPool{
            workers,
            sender,
        })
    }

    pub fn execute<F>(&self, _f: F)
        where
            F: FnOnce() + Send + 'static,
    {
    }
}
