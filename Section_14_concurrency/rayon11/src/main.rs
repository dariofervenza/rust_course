use rayon::prelude::*;
use num:: {BigUint, One};
use std::time::Instant;  // use it to track how much time does the sequential and the parallet process take (measure time functions take to run)

fn factorial(num: u32) -> BigUint {  // if we use u128, our factorial wont be able to go more than 35
    if num == 0 || num == 1 {
        return BigUint::one()
    } else {
        (1..=num).map(BigUint::from).reduce(|acc, x| acc * x).unwrap()  // the reduce is a closure with 2 arguments, 1. is the accumulator and 2. is the value vbeing iterated (we are returning so no ;)
    }
}

fn multi_thread_factorial(num: u32) -> BigUint {
    if num == 0 || num == 1 {
        return BigUint::one()
    } else {
        (1..=num).into_par_iter().map(BigUint::from).reduce(|| BigUint::one(), |acc, x| acc * x)  // into parallel iter. ALSO this reduce is from rayon and takes 2 closures, the first is the identity
        // the identity is going to be where we start out at ( the accumulator starts in 1), its just a different signature provided by rayon
        // also you DO NOT have to unwrap()
    }
}

fn main() {
    println!("Hello, world!");

    // rayon is a lightweight data parallel laser crate that makes it super easy to convert sequential computations into parallel computations while guaranteeing safety fron data races
    // it will auto spawn the threads for us

    // How does it know how many to spanw?? Uses the number of logiccal cpu's by default as the number of threads

    // cargo add rayon num

    let x: u32 = 3;

    println!("{} factorial is {}", x, factorial(x));

    let y: u32 = 3;
    println!("{} factorial is {}", y, multi_thread_factorial(y));

    // PERFORMANCE ANALYSIS

    let now = Instant::now();
    factorial(50000);
    println!("Sequential took {:.2?}", now.elapsed());
    let now = Instant::now();
    multi_thread_factorial(50000);
    println!("Parallel took {:.2?}", now.elapsed());

    let now = Instant::now();
    multi_thread_factorial(5000000);
    println!("Parallel extra took {:.2?}", now.elapsed());




}
