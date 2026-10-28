// Topic: calculateing_with_functions
// Run with:  cargo run --bin calculateing_with_functions

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // Channel = queue for sending jobs to workers
    let (sender, receiver) = mpsc::channel::<String>();

    // Multiple workers need access to the same receiver.
    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::new();

    // Create 4 worker threads
    for worker_id in 1..=4 {
        let receiver = Arc::clone(&receiver);

        let handle = thread::spawn(move || {
            loop {
                // Get the next job
                let job = {
                    let receiver = receiver.lock().unwrap();
                    receiver.recv()
                };

                match job {
                    Ok(filename) => {
                        println!("Worker {worker_id} processing {filename}");

                        // Pretend processing takes time
                        thread::sleep(Duration::from_millis(500));

                        println!("Worker {worker_id} finished {filename}");
                    }

                    Err(_) => {
                        println!("Worker {worker_id} shutting down");
                        break;
                    }
                }
            }
        });

        workers.push(handle);
    }

    // Send jobs
    for i in 1..=10 {
        let filename = format!("file_{i}.txt");

        sender.send(filename).unwrap();
    }

    // IMPORTANT:
    // Drop sender so workers eventually receive Err
    drop(sender);

    // Wait for all workers
    for worker in workers {
        worker.join().unwrap();
    }

    println!("All work completed!");
}
