use std::sync::{ Arc, Mutex };  // arc is used to share between threads

// external libs for concurrency
use crossbeam::channel::unbounded;
use std::thread;

use rayon::prelude::*;

fn main() {
    println!("Hello, world!");

    // to communicate between threads, there is more than msg passing

    // we can use MUTEXES: a mutex (mutual exclusion) -> only allows one thread to access some data at any time

    // to allow it, there is a lock: the lock is part of the mutex structure that keeps track of who currently has exclusive access to the data
    // when the thread wants to access the data, it asks the mutex for a lock and when it is acquired, only that thread can use the data --> UNTIL THE LOCK IS GIVEN BACK

    // LOCK RULES:
    // 1. you must acquire before using the data
    // 2.  you must give the lock back when finished

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..20 {
        let counter = Arc::clone(&counter);  // we have to create a copy because the original counter cant go to all threads, so we create a pointer to the same data
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // here the counter is moved to the thread (we dont clone the data but the pointer to it)
            *num += 1;
        });  // here the lock is released
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("After finishing all threads: {}", counter.lock().unwrap());


    // DEAD LOCK EXAMPLE:
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    println!("DEADLOCK EXAMPLE");
    for _ in 0..20 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            // lock twice
            // let mut num2 = counter.lock().unwrap();  // here the deadlock occurs
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("After finishing all threads: {}", counter.lock().unwrap());


    // this is all for concurrency, there a couple or crates: Cross Beam and Ray on --> check them to understand concurrency

    // CROSSBEAM: Gives us multiproducers and multiconsumer channels and to borrow variables

    
    let (sender, receiver) = unbounded();
    thread::spawn(move || {
        sender.send("Hello from thread using crossbeam").unwrap();
    });

    let msg = receiver.recv().unwrap();
    println!("{}", msg);


    // RAYON: parallel iterators, data parallelism, run computations in multiple threads

    let nums = vec![1, 2, 3, 4, 5, 6];
    let squared: Vec::<_> = nums.par_iter()
                              .map(|x| x * x)
                              .collect();  // with par_iter we get the parallel execution of vector.map() without creating anything else
    println!("{:?}", squared);


}
