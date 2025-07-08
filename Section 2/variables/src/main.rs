fn main() {
    println!("Hello, world!");
    let mut x: i32 = 56;
    x += 1;
    const CONSTANT: &str = "iamastringsconstant";
    println!("CONSTANT is {}", CONSTANT);
    println!("x is {}", x);
    let decimal = 02_69;
    println!("decimal is {}", decimal);
    let hex = 0xff;
    println!("hex is {}", hex);
    let oct = 0o377;
    println!("oct is {}", oct);
    let binary = 0b1111_1111;
    println!("binary is {}", binary);
    let byte = b'A';
    println!("ACSII VALUE OF A is {}", byte);

    let x = 2.0; // f64
    println!("x is {}", x);
    let valf32 = 3.3 as f32;
    println!("valf32 is {}", valf32);
    let boolean1: bool = true;
    println!("boolean1 is {}", boolean1);
    let c = 'c';
    println!("c is {}", c);

    let a = 10;
    let b = 4;
    let remainder = a % b;
    println!("remainder is {}", remainder);


}
