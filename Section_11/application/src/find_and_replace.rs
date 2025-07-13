use text_colorizer::*;
use std::env;  // to allow input values from the cli
use std::fs;
use regex::Regex;

#[derive(Debug)]  // allows us to use the std output easily
#[allow(dead_code)]  // it wont warn us when we dont use anything
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String
}


fn print_help(){
    eprintln!("{} - replace a string with a new string", "Find and replace".green());  // prints to the std error instead of the std out (use it for error and progress messages)
    eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}

fn parse_args() -> Arguments {
    println!("Hello, world!");

    // add dependencies to cargo.toml --> text-colorizer = "1" regex ="1"

    // in rust all versions 1.2 will be compatible with 1.3 and so on

    // but when we reach 2.0, they do not have to be compatible
    // also, when we tell version "1" it will load the lastest, eg 1.5.3 without using 2.0 

    // our program will take 4 arguments, a string to search for, a strign to replace it with, an input file and an output file
    print_help();

    let args: Vec::<String> = env::args().skip(1).collect(); // skip first argument that is the program name
    if args.len() != 4 {
        print_help();
        println!("{} wrong number of arguments. Expected 4, got {}", "Error".red().bold(), args.len());
        std::process::exit(1);  // finish the current program with code 1
    }
    Arguments{
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}

fn replace(target: &str, rep: &str, data: &str) -> Result::<String, regex::Error> {  // return a Result enum that contains a strign and the error is type regex::Error
    let regex = Regex::new(target)?; // propagate if fails
    Ok(regex.replace_all(data, rep).to_string())
}

fn read_and_write(args: &Arguments){
    let data = match fs::read_to_string(&args.input_file){
        Ok(f) => f,
        Err(e) => {
            print_help();
            eprintln!("{} failed to read file {}: {:?}", "Error".red().bold(), args.input_file, e);
            std::process::exit(1);
        }
    };
    let replace_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    };
    match fs::write(&args.output_file, &replace_data) {
        Ok(_) => {
            println!("{} to write {}", "Success".green(), args.output_file);
        },
        Err(e) => {
            eprintln!("{} failed to write file {}: {:?}", "Error".red().bold(), args.output_file, e);
            std::process::exit(1);
        }
    }
}

pub fn run(){
    let args = parse_args();
    println!("Args: {:?}", args);
    read_and_write(&args);

}