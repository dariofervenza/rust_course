use std::sync:: {Arc, Mutex};

fn main() {
    println!("Hello, world!");

    // what happens if a thread panics while we have a mutex in lock??
    // this is mutex poisoning --> very undesirable because all the other threads can not access the lock

    let lock = Arc::new(Mutex::new(0));
    let lock2 = Arc::clone(&lock);

    let _ = std::thread::spawn(move || {
        let _guard = lock2.lock().unwrap(); // acquire lock here
        panic!();  // mutex poisoned (panic!() while holding lock)
    }).join();

    let mut guard = match lock.lock() {  // provided that lock and lock2 point to the same data, our mutext is poisoned
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),  // recover the mutex with into_inner and allows us to access the data
    };
    *guard += 1;
    println!("Guard is {}", guard);  // it says guard is 1 because we could recover the value


    // therefore add this match if you think a thread may panic while holding a lock
}
