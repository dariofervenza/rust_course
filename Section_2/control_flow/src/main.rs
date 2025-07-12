fn main() {
    println!("Hello, world!");
    // if
    let one = 1;

    if one > 10 {
        println!("True");
    } else if one == 1 {
        println!("equal")
    } else {
        println!("False");
    }
    // basic loop
    loop {
        println!("loop");  //infinite
        break;
    }
    // give a name to a loop
    let mut num = 0;
    'counter: loop {
        println!("Count: {}", num);
        let mut decrease = 5;

        // nested loop
        loop {
            println!("Decreasing: {}", decrease);
            if decrease == 4 {
                break;
            }
            if num == 2 {
                break 'counter;  // breaks the outer named loop
            }
            decrease -= 1;
        }
        num += 1;
    }
    // while loop
    let mut num = 0;
    while num < 5 {
        println!("Num is: {}", num);
        num += 1;
    }

    // for loop --> loop over elemetns of a collection, eg vectors arrays
    let vec: Vec<i8> = (0..10).collect();
    for element in vec {
        println!("{}", element);
    }
    println!("end of for");
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("end of for");
}
