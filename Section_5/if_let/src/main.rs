enum Pet {
    Dog,
    Cat,
}

fn main() {
    println!("Hello, world!");

    // if let are equivalents to a match with only one option, but it can contain an else to allow other patterns
    let dog2 = Some(Pet::Dog);
    if let Some(Pet::Dog) = dog2 {
        println!("The animal is a dog");
    } else {
        println!("Its not a dog");
    }

}
