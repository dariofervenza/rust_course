use std::fs::File;
use std::fs::rename;
use std::io::Error;


fn main() {
    println!("Hello, world!");

    // if we try to match the error, it will result in a lot of match statements as we saw
    // we can propagate the error up the call stack with ?
    // we can use question mark ? to any expression that produces a Result enum
    let test = open_file();
    // test.unwrap();  // if panics here, instead of the function call, this is the propagation

    rename_file().unwrap();

    // however, in the most cases you would like to use expect() in the main thread instead of unwrap()

}

fn open_file() -> Result<File, Error> {
    let file = File::open("error.txt")?;  // with the ? we propagate the error
    Ok(file)
}


// renaming a file can fail as well

fn rename_file() -> Result<(), Error>{
    let file = rename("error.txt", "renamed.txt")?;
    Ok(file)
}