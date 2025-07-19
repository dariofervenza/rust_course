fn main() {
    println!("Hello, world!");

    // its important to organize our code as it grows, lets se which features rust includes to help us with this
    // these are: packages, crates, modules and paths

    // USE PACKAGES to build, test and share crates --> They are one or more crates that provide functionality
    // they have inside a cargo.toml file with instructions on how to build those crates,
    // indeed, every time we used cargo new we created a cargo.toml
    // if there are dependencies in our code, they appear there



    // CRATES are a trio of modules that produces a library or executable --> used to keep related functionalities together so we can use them later

    // MODULES lets us control the organisation, scope and privacy of paths

    // PATHS are a way to name an item such as a struct, a function or a module.


    // Example: remember that crates are to share code between projects WHILE Modules are to share code within ONE project

    // Modules are a collection of items such as named features eg: structs and functions
    
    // CREATE A LIBRARY: cargo new todo --lib
    // now it creates in src/lib.rs instead of main.rs

    // OPEN my_library/src/lib.rs to contiue there

}
