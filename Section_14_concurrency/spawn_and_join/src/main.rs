// import thhreading
use std::thread;
// send the main thread to sleep
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    // spliting the program into multiple threads might improve performance since we are taking advantage of multiple tasks at the same time

    // but this increases the complexity with issues like race conditions (threads are accesing data or resources inconsistently)

    // or deadlocks (when 2 threads are waiting for each other to finish, so neithero one finishes)

    // CREATE THE THREAD

    std::thread::spawn(move || {
        println!("Hello i am thread");
    });  // the main thread is finishing before this one so it is not printed

    // send the main thread to sleep
    thread::sleep(Duration::from_secs(1));
    println!("Main thread finished sleeping");

    // Alternative: use join to wait for the htead to finish

    // spawn RETURNS a JoinHandle<T>
    let handle = std::thread::spawn(move || {
        println!("Hello i am thread  that joins");
    });  // the main thread is finishing before this one so it is not printed

    handle.join().unwrap();  // RETURNS Result<T> (with Ok + Err)

    // SENDING VALUES TO MAIN

    let v = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {  // move allows us to the ownership of the values it uses, so the closure will take ownership of v and it will not outlast v
        println!("{:?}", v);
    });

    // SPAWNING MULTIPLE threads

    let mut thread_handles = Vec::new();
    let v = vec![1, 2, 3];
    for e in v {
        thread_handles.push(std::thread::spawn(move || println!("thread: {}", e)));
    }

    println!("I am the main thread");
    for handle in thread_handles {
        handle.join().unwrap();
    }





}
