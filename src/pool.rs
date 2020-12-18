// use crossbeam_utils::thread;
// use std::sync::mpsc;
// use std::sync::Arc;
// use std::sync::Mutex;

// type Job = Box<dyn FnOnce() + Send + 'static>;

// enum ThreadMessage {
//     Execute(Job),
//     Terminate,
// }

// pub struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<ThreadMessage>,
// }

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         for _ in &self.workers {
//             self.sender.send(ThreadMessage::Terminate).unwrap();
//         }
//         for worker in &mut self.workers {
//             if let Some(thread) = worker.working_thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }

// impl ThreadPool {

//     pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
//         let job = Box::new(f);
//         self.sender.send(ThreadMessage::Execute(job)).unwrap();
//     }

//     pub fn new(capacity: usize) -> ThreadPool {
//         assert!(capacity > 0);
//         let (sender, receiver) = mpsc::channel();
//         let receiver = Arc::new(Mutex::new(receiver));
//         let mut workers = Vec::with_capacity(capacity);
//         for id in 0..capacity {
//            let worker = Worker::new(id, Arc::clone(&receiver));
//            workers.push(worker);
//         }
//         ThreadPool { workers, sender }
//     }

// }

// struct Worker {
//     id: usize,
// }

// impl Worker {

//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<ThreadMessage>>>) -> Worker {
//          thread::scope(|s| {
//              let handle = s.spawn(|_| { 
//                  loop {
//                      let thread_message = receiver.lock().unwrap().recv().unwrap();
//                       //Execute Job or Terminate Thread
//                       match thread_message {
//                           ThreadMessage::Execute(job) => {
//                               job();
//                           },
//                           ThreadMessage::Terminate => {
//                               break;
//                           }
//                       }
//                  }
//              });
//         });
//         Worker { id }
//     }

// }