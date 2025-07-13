use std::cell::RefCell;
use std::rc::Rc;

struct Flagger {
    is_true: RefCell::<bool>
}
struct Flagger3 {
    is_true: Rc::<RefCell::<bool>>
}


fn main() {
    println!("Hello, world!");

    // reference cell (RefCell) allows us to follow the interior mutability pattern 

    // interior mutability is a design pattern that allows us to mutate data even when there are inmutable references to that data
    // this is typically not allowed as we saw earlier

    // therefore this uses unsafe code to bend rust usual rules of mutation and borrowing

    // rust self rules are enforced at runtime, so the compiler wont catch any errors for us
    // but if an error is present at runtime, then the program will panic and terminate
    let flag = Flagger{is_true: RefCell::new(true)};
    // when we use a RefCell we gain access to 2 new methods: Borrow and borrow mute

    // BORROW: returns the smart pointer as a reference to t so borrow returns Ref<T>
    // BORROW MUTE: returns RefMut<T>

    // both of these implement derrefencing so we can treat them as regular references

    let reference = flag.is_true.borrow();
    println!("{}", reference);  // we expect true to be printed


    // we can not create 2 references so we have to recreate flag
    let flag = Flagger{is_true: RefCell::new(true)};
    let mut mut_ref = flag.is_true.borrow_mut();
    *mut_ref = false;  // we derrefence it to modify it
    println!("{}", mut_ref);
    // it CHANGED flag, so we mutated an inmutable value

    // we can not have multiple borrows unless we pair it with an Rc --> lets see it

    let flag = Flagger3{is_true: Rc::new(RefCell::new(true))};
    let reference = Rc::new(flag.is_true.clone());  // when doing clone, is_true is a Rc, so we are creating another Rc pointer to another pointer RefCell. Remember that Rc maintains the methods like borrow_mut 
    println!("{:?}", reference);

    let mut mut_ref = reference.is_true.borrow_mut();
    *mut_ref = false;
    println!("{}", mut_ref);

    // when to choose Box, Rc or RefCell ??

    //  Rc enables multiple owners of the same data
    // box and refcell have one owner
    // box allows inmutable or mutable borrows checked at compile time
    // Rc allows only inmutable borrows checked at compile time
    // RefCell allows mutable and inmutable borrows checked at RUNTIME

}
