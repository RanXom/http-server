#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSize,
}

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::ZeroSize)
        }

        Ok(ThreadPool)
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        
    }
}
