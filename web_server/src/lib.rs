use std::sync::{mpsc,Arc,Mutex};
use std::thread;

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size>0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }

        ThreadPool{
            workers,
            sender
        }
    }
    pub fn execute<F>(&self,f:F)
        where F: FnOnce() + Send + 'static {
        //Usiamo come constraint per il template FnOnce() con tanto di parentesi per specificare
        //che deve ritornare (), ovvero il tipo unitario
        
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers{ //qui stiamo facendo si che tutti i vari thread siano spenti di
            //modo che non possano più ricevere richieste
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers{ //una volta spenti tutti i thread, andiamo a levarli dai
            //worker.
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.",id);
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.",id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread)
        }
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

trait FnBox {
    fn call_box(self: Box<Self>);
}
impl<F: FnOnce()> FnBox for F{
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

enum Message{
    NewJob(Job),
    Terminate
}
