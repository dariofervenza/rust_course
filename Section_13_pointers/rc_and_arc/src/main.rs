use std::rc::Rc;

fn main() {
    println!("Hello, world!");

    // in most cases ownership is clear, we know which variable owns a value
    // but there are cases where a single value might have or need multiple owner

    // example graph data structure: in a graph each node is connected to other nodes so each node has multiple owners

    // Rc: Allows us to have multiple owners (reference counted)
    // Rc is also allocated on the heap and it tracks the number of references to evaluate whether or not a value is still in use
    // if there are zero references, the value is cleaned up

    // ArkRc: Atomic reference counting --> is the same as Rc but it is designed for safe sharing between threads, so only use it when sharing the pointer between threads (otherwise stick to Rc)

    // example: first bring it into scope (import)

    let s1 = Rc::new(String::from("pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();
    // these 3 point all to the same value, to probe it we will print them out
    println!("{}, {}, {}", s1, s2, s3);

    // so we have a Rc value pointer to a heap allocated string, which is going to have a reference count associated with it
    // cloning an Rc does not copy the value, it creates another pointer to it and increments the reference count
    // in this example the string pointer has a reference count of 3 and s1, s2 and s3 which are in the stack, all point to a string in the heap

    // with the data being a string, we are still able to use string methods as usual directly
    println!("{}, {}, {}", s1.contains("point"), s2, s3.contains("ter"));  // we expect to print: true, pointer, true


}
