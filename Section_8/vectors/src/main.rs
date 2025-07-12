use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    println!("Hello, world!");
    // lets see vectors and some of the methods associated with them
    let mut nums: Vec<i32> = vec![];

    // add values with push (like append)
    nums.push(6);
    nums.push(632);
    nums.push(43);
    nums.push(452);
    nums.push(76);
    nums.push(4396);
    nums.push(78);

    // remove values of the top with pop

    let top = nums.pop();  // it will return Option<T> so None or Some(value)
    println!("{}", top.unwrap());

    let two = nums[1];  // because i32 implements copy, this will be a copy

    let two_ref = &nums[1]; // if copy is not available, the compiler automatically implements a reference

    // get the first value with fisrt

    let one = nums.first();  // returns a Option<T> also, it will be None if the vector is empty or Some(value)
    println!("first val is {}", one.unwrap());

    // get the last element with last()

    let last = nums.last();
    println!("last val is {}", last.unwrap());

    // there are also a .first_mut() and a .last_mut() methods --> they will borrow mutable references

    // get the length with .len()

    println!("len of nums is {}", nums.len());

    // check if it is empty with .is_empty()
    println!("nums is empty? {}", nums.is_empty());  // returns a bool

    // add values but in other positions

    nums.insert(1, 999);  // index, value
    nums.insert(3, 888);  // index, value
    println!("nums is {:?}", nums);

    // remove values with .remove()

    nums.remove(3);  // specify the index
    println!("nums without element 3 is {:?}", nums);

    // sort it 

    nums.sort();
    println!("nums sorted is {:?}", nums);

    // reverse it

    nums.reverse();
    println!("nums sorted but reversed is {:?}", nums);

    // shuffle the vector --> go to cargo.toml and update the dependencies with rand = "0.8.4", then import with use in the top
    nums.shuffle(&mut thread_rng());
    println!("nums shuffled is {:?}", nums);

}
