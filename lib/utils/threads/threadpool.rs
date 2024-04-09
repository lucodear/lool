use {
    crate::fail,
    core::fmt,
    eyre::Result,
    std::{
        fmt::{Debug, Formatter},
        sync::{
            atomic::{AtomicUsize, Ordering},
            mpsc::{channel, Receiver, Sender},
            Arc, Mutex,
        },
        thread,
    },
};

type Job = Box<dyn FnOnce() + Send + 'static>;

/// status of the thread pool
struct PoolStatus {
    /// the number of pending jobs in the queue
    queued_count: AtomicUsize,
    /// the number of currently active jobs
    active_count: AtomicUsize,
    /// workers count
    pool_size: usize,
}

impl PoolStatus {
    fn create(pool_size: usize) -> PoolStatus {
        PoolStatus {
            queued_count: AtomicUsize::new(0),
            active_count: AtomicUsize::new(0),
            pool_size,
        }
    }

    /// returns `true` if has active work (either queued or running)
    fn has_work(&self) -> bool {
        self.queued_count.load(Ordering::SeqCst) > 0 || self.active_count.load(Ordering::SeqCst) > 0
    }
}

/// 🧉 » a simple thread pool implementation
///
/// this is based on the example from the *"The Rust Programming Language"* book, with some
/// modifications inspired by the [threadpool](https://crates.io/crates/threadpool) crate.
///
/// unlike the original implementation from the book, this one doesn't wait for all jobs to finish
/// when the pool is dropped. Instead, it's up to the user to call `join` to wait for all jobs to
/// finish before dropping the pool. This allows the user to stop the pool and drop immediately if
/// that's what they want
///
/// **why?**
///
/// the implementation from the book was thought to be used in a web server where it's
/// important to wait till all active requests are finished before shutting down the server.
///
/// In other scenarios, though, this might not be necessary. For example, if we have a long-running
/// process and we need to shut down the pool, we might want to implement our own logic on how to
/// handle the shutdown. e.g. We might wait for some jobs, but not all of them, etc.
pub struct ThreadPool {
    workers: Vec<Worker>,
    job_sender: Option<Sender<Job>>,
    status: Arc<PoolStatus>,
}

impl ThreadPool {
    /// 🧉 »  creates a new `ThreadPool`.
    ///
    /// The `capacity` is the number of threads in the pool.
    ///
    /// **Errors**
    ///
    /// If the `capacity` is zero, an error is returned.
    pub fn create(capacity: usize) -> Result<ThreadPool> {
        if capacity == 0 {
            return fail!("ThreadPool size cannot be zero.");
        }

        let status = Arc::new(PoolStatus::create(capacity));

        let (job_sender, job_receiver) = channel();
        let receiver = Arc::new(Mutex::new(job_receiver));
        let mut workers = Vec::with_capacity(capacity);

        for id in 0..capacity {
            workers.push(Worker::new(id, receiver.clone(), status.clone()));
        }

        Ok(ThreadPool {
            job_sender: Some(job_sender),
            workers,
            status,
        })
    }

    /// 🧉 » queues execution of a task/function in the thread pool
    ///
    /// the function `f` will be executed as soon as a worker thread is free.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(ref job_sender) = self.job_sender {
            self.status.queued_count.fetch_add(1, Ordering::SeqCst);
            job_sender
                .send(Box::new(f))
                .expect("ThreadPool::execute unable to send job into queue.");
        } else {
            panic!("ThreadPool::execute called, but there's no job_sender... weird!");
        }
    }

    /// 🧉 » returns the number of active jobs
    pub fn active_jobs(&self) -> usize {
        self.status.active_count.load(Ordering::SeqCst)
    }

    /// 🧉 » returns the number of queued jobs
    pub fn queued_jobs(&self) -> usize {
        self.status.queued_count.load(Ordering::Relaxed)
    }

    /// 🧉 » returns the pool size
    pub fn pool_size(&self) -> usize {
        self.status.pool_size
    }

    /// 🧉 » returns `true` if the pool has active work (either queued or running)
    pub fn has_work(&self) -> bool {
        self.status.has_work()
    }

    /// 🧉 » waits for all threads to finish their work
    pub fn join(&mut self) {
        drop(self.job_sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Debug for ThreadPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ThreadPool")
            .field("queued_jobs", &self.queued_jobs())
            .field("active_jobs", &self.active_jobs())
            .field("pool_size", &self.pool_size())
            .finish()
    }
}

/// the worker is responsible for executing the jobs on a specific thread.
///
/// it holds its own thread and waits for a job to be sent to it from the `ThreadPool::execute` fn.
/// Once it receives a job, it
/// executes
/// it and then waits for another job, until the `job_receiver` is dropped (usually when the pool
/// is dropped).
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        job_receiver: Arc<Mutex<Receiver<Job>>>,
        pool_status: Arc<PoolStatus>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = {
                // Only lock jobs for the time it takes to get a job, not to run it.
                let lock = job_receiver.lock().expect("Worker thread unable to lock job_receiver");
                lock.recv()
            }; // drops the lock here and now others are free to get another job

            let job = match message {
                Ok(job) => job,
                Err(..) => break,
            };

            pool_status.active_count.fetch_add(1, Ordering::SeqCst);
            pool_status.queued_count.fetch_sub(1, Ordering::SeqCst);

            job();

            pool_status.active_count.fetch_sub(1, Ordering::SeqCst);
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
