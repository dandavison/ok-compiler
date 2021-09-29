use std::sync::mpsc;
use std::thread;
use std::time;

pub fn ok_thread() {
    let join_handle = thread::spawn(|| println!("Hello from a thread."));
    join_handle.join().unwrap();
}

pub fn ok_thread_move() {
    let data = 7;
    let join_handle = thread::spawn(move || println!("Hello from a thread. data is {}", data));
    join_handle.join().unwrap();
}

// produce some values, send them into a channel, and consume them.
// Rust implementation of go_concurrency_exercise_1.py
pub fn ok_channel() {
    let (sender, receiver) = mpsc::channel();

    // A thread that emits a stream of data into the channel
    let producer_thread = thread::spawn(move || {
        let mut i = 0;
        loop {
            sender.send(i).unwrap();
            thread::sleep(time::Duration::from_secs(1));
            i += 1;
        }
    });

    let consumer_thread = thread::spawn(move || {
        for item in receiver {
            println!("Got item: {}", item);
        }
    });

    producer_thread.join().unwrap(); // should never return
    consumer_thread.join().unwrap();
}

pub fn main() {
    ok_thread();
    ok_thread_move();
    ok_channel();
}
