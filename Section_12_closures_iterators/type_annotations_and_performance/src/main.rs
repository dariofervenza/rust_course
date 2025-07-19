fn main() {
    println!("Hello, world!");

    // closures do not require to use type annotations because they are typically short and have a narrow context --> the compiler infers the types

    // but we can still annotate

    let add = |x: i32| -> i32 {x + 1};
    let add2 = |x| x + 1;  // requieres type annotations unless we use it:

    add2(5);

    // closure with 1 arg that returns it

    let example = |x| x;

    let string = example(String::from("hello"));
    // NOTE
    // let num = example(5);  // this wont work because in the previous run it infered string type for x

    // CLOSURES are fast, they are not allocated in the head unless you use a container like a vector


}
