fn main() {
    println!("Hello, world!");
    // stack and heap
    // stack: stores values in the order it gets them and remove in the oposite --> last in first out
    // data in the stack must have a known fixed size

    // heap: when you put something on it, you request a space and it returns a pointer (address of that location)

    // pushing to the stack is faster than allocating on the heap --> search data on the stack is faster than the heap

    // when you pass values into a function or you have local variables in a function, they are pushed to the stack
    // when the function is over, they are popped out of the stack

    // EACH value in rust has a variable called its owner at a time (only one at a time)

    // when the owner goes out of scope, the value is dropped (free)

    let var = 1;  // its a i32, fixed size so its pushed to the stack
    let mut s = "hello".to_string();  // mutable so created on the head
    s.push_str(", world");


    // WHAT IS A MOVE?

    // when you do a move, you move the ownership from one variable to another

    let x = vec!["tyler".to_string()];
    let y = x;
    // println!("{:?}", x); // this triggers an error because in the previous line we gave the ownership of x to y, x no longer owns that value

    // therefore, a value can belong only to one variable

    // VALUE COPIES TO AVOID THIS

    // what is a clone? If we want to assign a value to another variable without changing the ownership, use clone() --> deep copy
    let x = vec!["tyler".to_string()];
    let y = x.clone();
    println!("{:?}", x);


    // WHAT IS A COPY?

    let x = 1;
    let y = x;
    println!("x is {}, y is {}", x ,y);  // why does this work?

    // most types implement a move, some of then implement a copy
    // a copy will be implemented on types that are stored in the stack, integer, boolean, a tuple if every value it contains implement copy
    // but a vector does not implement a copy, that is why we have to do the clone

    // MORE MOVES

    let s = String::from("takes");
    takes_ownership(s);  // here we take the ownership from s to strin
    // we pass the ownership when we pass variable thorugh parameters

    // paassing a vlaue as a copy
    let val = 1;
    make_a_copy(val);
    println!("val is {}", val);  // i32 implements the copy move
    

    // receive ownership
    let str1: String = give_ownership();
    println!("str1 is {}", str1);

    let str3: String = take_and_give(str1);
    println!("str3 is {}", str3);  // str1 has lost ownership`
    
    // what happens with control statements?
    
    if true {
        let str4 = str3;
    } else {
        let str5 = str3;
    }
    // println!("str3 is {}", str3);  // this gives an ownership error 

    // CONCLUSION: in the control statements, ownership is also lost
    let mut str1 = String::from("tyler");
    let mut str2: String;

    loop {
        str2 = str1;
        break // if we do not break, IN THE SECOND LOOP, str1 has no ownership and triggers an error
    }

    // REFERENCES AND BORROWING

    // use references to give a reference to a value without taking ownership (we are borrowing the value)

    // 2 types of references

    // a) shared reference: allows reading but no modifying what is referenced --> you can have many shared references to one value at a time

    // b) mutable references: reading + modifying but only one reference per value at a time, you have to deactivate one to create another


    let mut s = String::from("hello");
    change_string(&mut s);
    println!("s is {}", s);
    // if we didtn pass a reference, we would lose the ownership

}

// here var is dropped and also s is dropped, the memory is released to the operating system

fn takes_ownership(s: String) {
    let strin = s;
    println!("strin is {}", strin);
}

fn make_a_copy(one: i32) {
    let val1 = one;
    println!("val1 is {}", val1);
}

fn give_ownership() -> String {
    "given".to_string()
}

fn take_and_give(str2: String) -> String {
    str2
}

fn change_string(some_string: &mut String){  // is &mut instead of mut some_string because its a reference, we can not borrow as mutable 
    some_string.push_str(", world");
}