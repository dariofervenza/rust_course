use std::collections::HashMap;
// use std::collections::BTreeMap;

fn main() {
    println!("Hello, world!");

    // a map is a collection of key-value pairs, where there cant be 2 elements with the same key
    // also you need the key to know the value of an entry


    // there are 2 TYPES OF MAPS: hash map and B tree map (binary tree map)
    // they are very similar but differ mainly in how they keep entries arranged for their lookup

    // HASH MAPS are stored in a single head allocated table
    // BTREE MAPS are  stored in a tree like structure growing from the top down with many leaves ( nodes)

    // they also have very similar methods so we are going to see a hash map

    let mut hmap = HashMap::new();

    // we will have also a .len() and a .is_empty() method like in vectors

    // add values with insert

    hmap.insert(1, 1);  // key, value
    hmap.insert(5, 50);
    hmap.insert(9, 50);
    hmap.insert(10, 510);
    let old_value = hmap.insert(10, 1);  // the value in key 10 get overwritten and also returs the old value Some(510) --> it returns an option<T>
    println!("Hmap is {:?}", hmap);

    // check if a key exists iwth contains_key(&val)
    println!("Does h map contain 2 as key? {}", hmap.contains_key(&2));

    // how to get values back?
    println!("Value for key 9 is: {:?}", hmap.get(&9));  // returns Some(50)

    // remove values

    let val = hmap.remove(&1); // remove the key of 1, but also retuns the removed value associated to that key as Option<T>
    println!("Hmap is {:?}", hmap);

    // remove and get the pair key-value with remove_entry (using the key)

    let remove = hmap.remove_entry(&9);  // as Some((key, val))
    println!("remove is {:?}", remove);

    // clear out the hash map

    hmap.clear();
    println!("Hmap is empty?? {}", hmap.is_empty());

}
