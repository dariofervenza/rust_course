fn main() {
    println!("Hello, world!");
    // first way to create an string
    let name = String::from("Paco");
    println!("name is {}", name);
    let course = "RUST".to_string();
    println!("course is {}", course);
    // modify strings
    let new_name = name.replace("Paco", "Manolo");
    println!("new_name is {}", new_name);

    // string slice = &str or (stir) --> will (like in vectors) reference and borrow  the text from the variable (so its a fact point and contains both the address and  the actual data) --> you can not modify a string slice
    let str1 = "hello";  // it will be a string slice (&str)
    // println!("string bogus {}", str1.bogus());  // --> triggers errro because its a slice and can not be modified ( that method is not present in a string slice)
    
    // when to use String or string slice? 

    // flip flop between Strings and &str
    let str2 = str1.to_string();
    println!("str2 is {}", str2);
    let str2_slice = &str2;
    println!("str2_slice is {}", str2_slice);

    // compare strings with == or !=
    println!("{}", "ONE".to_lowercase() == "one");

    // check string methods

    // string literals -> Strings and &str are always a valid utf sequence, if you dont want a valid utf:

    let rust = "\x52\x75\x73\x74";  // special caracters
    println!("rust is {}", rust);






}
