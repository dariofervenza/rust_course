enum Pet {
    Dog,
    Cat,
}

fn main() {
    println!("Hello, world!");

    // if let are equivalents to a match with only one option, but it can contain an else to allow other patterns
    let dog2 = Some(Pet::Dog);
    if let Some(Pet::Dog) = dog2 { // with if let we dont use ==
        println!("The animal is a dog");
    } else {  // we could use else if also
        println!("Its not a dog");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {  // while there is some number, we will iterate
        println!("top is {}", top)
    }

}
