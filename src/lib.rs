use std::thread;

#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSize,
}

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::ZeroSize)
        }

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            let handle = thread::spawn(|| {

            });

            threads.push(handle);
        }

        Ok(ThreadPool{
            threads
        })
    }

    pub fn execute<F>(&self, _f: F)
        where
            F: FnOnce() + Send + 'static,
    {
    }
}
