use std::fs::File;
// import error kind

use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");
    // the easiest way to catch errors is to use the match statement
    let file = File::open("idonotexist.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("idonotexist.txt"){
                Ok(file_created) => file_created,
                Err(error) => panic!("Can not create the file: {:?}", error)
            },
            _ => panic!("unexpected error"),
        },
    };

    // the error kind is provided by the std library

    // SECOND WAY TO HANDLE ERRORS: unwrap()

    let file = File::open("idonotexist.txt").unwrap();  // if ok is returned, the unwrap with return the value inside the ok forest
    // if the result is Err, the unwrap() will panic!

    // Customize the error with: EXPECT
    let file = File::open("idonotexist22.txt").expect("ERROR OPENING the file");
    

}
