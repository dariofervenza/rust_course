use std::rc::Rc;
use std::sync::Arc;  // atomic reference counter to share ownership between threeads

fn main() {
    println!("Hello, world!");

    // we have to see 2 neew important traits: send and sync

    // types that implement send are safe to pass a value to another thread --> this means that they are free from data races and other undefined behaviour

    // types that that implement send --> can also be moved across threads

    // types that implement SYNC are safe to pass by non mutable reference to another thread, can be shared across threads so
    // send can be moved across threads and sync can be shared across threads 

    /// almost every type is send or sync

    // TYPES THAT ARE NOT SEND / SYNC: RC reference counter smart pointer

    // Example:

    let rc1 = Rc::new(String::from("Test"));
    let rc2 = rc1.clone();

    let arc1 = Arc::new(String:.from("this can be enter a thread"));
    std::thread::spawn(move || {
        // rc2;  // this triggers an error, Rc can not be moved into a thread
        arc1; // no error with this, atomic ref counter is safe to share between threads
    });

}
