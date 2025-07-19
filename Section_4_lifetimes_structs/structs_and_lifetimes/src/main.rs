struct MyString<'a> {
    text: &'a str
}

fn main() {
    println!("Hello, world!");
    // it is possible to store references in structs but we need to add a life annotation of every reference
    // we have to do StructName<'a> because when we create an instance because we want to ensure that the instance x does not outlive the referecen to the text that holds

    let str1 = String::from("this is a string");
    let x = MyString{text: str1.as_str()};

    // STATIC LIFETIMES:
    // this references can live throughout all the duration of the main program

    // All string literals have a static lifetime

    let s: &'static str = "I am a string literal with a static lifetime";  // will be directly in the programs binary, so it is always available

    // when do we want something live so long?
    // most lifetime errors will result from dangling references, so its better to fix it by assigning static lifetimes
}
