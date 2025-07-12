use std::collections::HashSet;

fn main() {
    println!("Hello, world!");

    // the set can not contain multiple copies of a value, they are similar to maps but they only contain keys

    // 2 types: HASH set and B tree sets
    let mut hset: HashSet<i32> = HashSet::new();

    // there are len() and is_empty() methods

    // insert 

    hset.insert(1);
    hset.insert(3);
    hset.insert(36);
    hset.insert(89);
    hset.insert(105);
    hset.insert(240);

    println!("Hset is {:?}", hset);

    // use an iterator

    for x in hset.iter() {
        println!("Hset val is {}", x);
    }

    // remove a value

    hset.remove(&36);
    println!("Hset is {:?}", hset);

    // INTERSECTION between 2 sets

    let mut hset2: HashSet<i32> = HashSet::new();
    hset2.insert(1);
    hset2.insert(2);
    hset2.insert(3);
    hset2.insert(8);
    hset2.insert(9);
    hset2.insert(240);

    for x in hset.intersection(&hset2) {
        println!("Intersection val: {}", x);
    }


    // UNION
    let unioon = hset.union(&hset2);
    println!("Union is {:?}", unioon);

    // DIFFERENCE
     let differ = hset.difference(&hset2);
    println!("Difference is {:?}", differ);   


    // NOTE: THERE ARE MORE COLLECTIONS IN THE STD RUST LIBRARY SO CHECK THEM
}
