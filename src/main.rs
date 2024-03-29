use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;
use std::time::Duration;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

   for _stream in listener.incoming() {
       let stream = _stream.unwrap();
      ThreadPool::spawn(|| {
          handle_connection(stream);
      })
   }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

struct Job;

impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {
           Worker {
               id,
               thread
           }
        });
    }
}

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        for id  in 0..size {
            threads.push(Worker::new(id));
        }
        ThreadPool {
            threads,
            sender
        }
    }

    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
        where
            F: FnOnce() -> T + Send + 'static,
            T: Send + 'static
    {}

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static

    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

type Job = Box<FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;

            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker has received {} job now executing...", id);
                job.call_box();
            }
        });

        Worker {
            id,
            thread
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get){
        let mut file = File::open("test.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n\r\n", "test.html")
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open("404.html").unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    };
}   
