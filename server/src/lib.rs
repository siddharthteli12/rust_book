use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(count: usize) -> Self {
        let mut workers = Vec::with_capacity(count);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..count {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Self { workers, sender }
    }

    pub fn execute<T>(&self, f: T)
    where
        T: FnOnce(),
        T: Send,
        T: 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
#[derive(Debug)]
pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>) -> Self {
        Worker {
            id,
            thread: thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Received new job, id - {id}");
                job();
            }),
        }
    }
}
