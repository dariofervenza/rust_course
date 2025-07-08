fn main() {
    println!("Hello, world!");
    print_phase();
    print_phase2("print my argument");
    println!("{}", greatest_common_denominator(20, 5));
    println!("{}", multiple_return(true));
    
}

fn print_phase() {
    println!("hello from the funct");
}

fn print_phase2(phrase: &str) {
    println!("{}", phrase);
}

fn greatest_common_denominator(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    b  // this means return b (without ;)
}

fn multiple_return(flag: bool) -> bool {
    if flag == true {
        true  // this is one return
    } else {
        false  // this is another return
    }
}