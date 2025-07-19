fn main() {
    println!("Hello, world!");

    // a pointer is a general concept for a variavle that contains an address in memory, which that address points to some data

    // we have already used pointers, the most common one is a reference (&)

    // that pointers do not give us additional capabilities

    // however there is one: SMART POINTERS that give us these additional capabilities

    // smart pointers originated in c ++ so they are not unique to rust

    // they are box which allocates values on the heap reference counting Rc (it is a reference counting type that allows multiple ownerships and ref)

    // BOX SMART POINTER: a box is the most straightforward pointer rust offers --> this allows us to allocate data on the heap rather than the stack
    // however, the pointer to the heap data is going to remain on the stack
    // example of using a box:
    let t = (12, "eggs");  // new tuple --> they are created on the stack
    let b = Box::new(t);  // created on the heap, but b is stored in the stack
    println!("{:?}", b);  // printing the box, prints the pointer

    // lets create a simple reference

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // assert_eq!(5, y);  // this triggers an error because we can not compare an integer to a reference
    // Solution: deallocate the reference --> * returns the value at that memory address
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{}", y);

    // In conclusion, boxes allow us to allocate data into the heap
}
