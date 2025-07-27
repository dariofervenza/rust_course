fn main() {
    println!("Hello, world!");

    // writing unsafe code gives more control but we have to be careful

    // we have to place our code inside an unsafe block

    let mut num = 5;
    let r1 = &num as *const i32;  // we expect this to be the memory address
    let r2 = &mut num as *mut i32; // same
    println!("r1 is {:?}", r1);
    println!("r2 is {:?}", r2);

    // if we try to dereference them we should get an error

    // println!("r1 dereferenced is {:?}", *r1);  // --> error: dereference of raw pointer --> the memory safety prevests it because can  cause data races

    // solution: do it in an unsafe code block
    unsafe {
        println!("r1 dereferenced is {:?}", *r1);
        println!("r2 dereferenced is {:?}", *r2);
    }

    // a function with unsafe code does not mean that all the function has to be unsafe, you can just put the unsafe block in the lines that are unsafe and not all

    // WHEN SHOULD I need unsafe code? what are the opportunities to use it? when i might need to use it
    // it allows you to bypass the safety checks that are enforced by the compiler

    // it may be iseful for tasks like interfacing low level system API's, hardware or when creating performance critical algorithms

    // example: interfacing with c libraries --> system API's may require unsafe code to interact with low level C types used on these api's

    // example: low level programming --> direct access to memory (custom memory allocator, low level data structure)

    // example: creating custom data structures, when creating them you require fine grained control over the memory allocation and deallocation

    // CATCH: It can lead to undefined behaviour or memory safety issues -> only use it when it is completely necessary

    // besides: rust provides many safety features to minimize the risk of unsafe code --> memory safety checks, the use of lifetimes and other borrow checker checks
}
