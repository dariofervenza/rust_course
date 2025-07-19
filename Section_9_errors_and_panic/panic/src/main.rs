// open files with rust
use std::fs:File;

fn main() {
    println!("Hello, world!");

    // there 2 types of errors --> recoverable and unrecoverable

    // recoverable errors are going to rely on the result type

    // unrecoverable ones are going to use the panic! macro 

    // PANIC MACRO: it is going to terminate the current thread

    // TRIGGER A PANIC:
    // panic!("panicked here");  // this triggers the error

    // Example of real panic: access an out of obucne element in an array

    let vec = vec![1];
    // vec[10];  // thread 'main' panicked at src/main.rs:17:8:

    // i do export RUST_BACKTRACE=1 and run it again with the out of bounds, it gives a trace

    // in panic, it unwinds, this is removes all the variables in the reverse order they were created, then goes to the function that called it and unwinds, when everything is crear it closes

    // ABORT: in the event of an abort the unwind does not happen
    // eg. we customized a drop method during the unwinding process, but the drop panics --> this is fatal and causes the whole process to drop


    // many errors do not cause the programm to stop, in that case we use the RESULT type
    // RESULT TYPE: it allow us to prepare in case there is an error
    // result is an enum with 2 variants: ok and air

    let file = File::open("error.txt");
    
}

// result definition
enum Result<T, E> {
    Ok(T),
    Err(E),
}
