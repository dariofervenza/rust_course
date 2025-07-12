fn main() {
    println!("Hello, world!");
    // match 2 conditions
    let x = 1;
    match x {
        1 | 2 => println!("The number is 1 or 2"), // OR operator
        _ => println!("the value is other number"),
    }

    // match on an incluse range of values
    let x = 5;
    match x {
        1..5 => println!("The number is in the range 1 to 4 both incluse"),
        _ => println!("The number is out of range 1 to 4 both incluse")
    }
    let x = 5;
    match x {
        1..=5 => println!("The number is in the range 1 to 5 both incluse"),
        _ => println!("The number is out of range 1 to 5 both incluse")
    }

    // if condition after the match

    let x = Some(5);
    let y = 5;
    match x {
        None => println!("Is none"),
        Some(10) => println!("Is some 10"),
        Some(x) if x == y => println!("The some(x) is {}", y),
        _ => println!("Neither"),
    } 
}
