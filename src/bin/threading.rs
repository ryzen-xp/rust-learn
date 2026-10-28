// Topic: threading
// Run with:  cargo run --bin threading

use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let s = Arc::new(String::from("Sandeep Hi !!"));

    thread::spawn(move || {
        tx.send(Arc::clone(&s)).unwrap();

        println!("{}", s);
    });

    let recived = rx.recv().unwrap();

    println!("This is what we get from spwan thread : {}", recived);

    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("spawn thread ::  {i} ");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("Default thread ::  {i}");
    //     thread::sleep(Duration::from_millis(1));
    // }
}
