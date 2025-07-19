// Option built in definition
// enum Option<T> {
//     None,
//     Some(T),  // T means that is generic, it represents a value that exists, and T is a the datatpye
// }

// this enum can be either None or Some(value)
// Option is a predefined enum han Some is a variant of Option used to wrap a value

// Option is like for instance in python Optional[str] 

fn main() {
    println!("Hello, world!");

    let some_number = Some(5);
    let some_string = Some("im a stirng");
    let nothing: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    // let sum = x + y;  // can not be added, y has to be converted

    // USING MATCH STATEMENT
    // match allows to compare a value against a series of patterns that execute code

    // we did it the Pet enum impl

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    // println!("{}", six); // cant print because it cant format it
    what_pet("Dog");
    what_pet("Cat");

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,  // if we dont include the None case, the compiler will detect it and disallow compilation, the compiler will check all the possibilities in the match
        Some(i) => Some(i + 1),  // if some is passed, we return Some of i + 1
    }
}

fn what_pet(input: &str) {
    match input {
        "Dog" => println!("I have a dog"),
        &_ => println!("Other pet"),  // we have to cover other possibilities or it wont run
    }
}
