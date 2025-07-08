-fn main() {
    println!("Hello, world!");

    // tuple (fixed length)
    let tup = (500, "hi", 3.68, true);
    println!("tup.0 is {}", tup.0);
    // unpack tuples
    let (a, b, c, d) = tup;
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
    println!("d is {}", d);

    // arrays,  also fixed length but each element has to be the same type as others
    let array = [1, 2, 3];
    println!("array[0] is {}", array[0]);
    // define length and type
    let array: [i32; 3] = [5, 6, 7];
    println!("array[0] is {}", array[0]);
    // modify values of the array
    let mut array: [i32; 3] = [5, 6, 7];
    array[0] = 50; 
    println!("array[0] is {}", array[0]);

    // vector: resizable
    let mut nums = vec![1, 2, 3];
    nums.push(4);
    println!("nums[3] is {}", nums[3]);
    println!("nums is {:?}", nums);  // with :? we foramt it so we can print it
    nums.pop();
    println!("after pop, nums is {:?}", nums);  // with :? we foramt it so we can print it

    // other way to create vectors
    let mut vec = Vec::new();
    vec.push("string");
    vec.push("second");
    println!("vec is {:?}", vec);

    // explore vector methods like:
    vec.reverse();
    println!("vec once reversed is {:?}", vec);

    //specify vector size

    let mut vect = Vec::<i32>::with_capacity(2);
    println!("vect capacity is {}", vect.capacity());

    // if we add elements to it, it creates a new vector and copies the content but with the added value

    // create a vector with a range

    let v: Vec<i32> = (0..5).collect();
    println!("v is {:?}", v);

    // slices: a range of a vector or an array, we can not save them in variables or pass them as function arguments
    let slice_v: &[i32] = &v;  // althougt we can not pass them into a variable, here we are point slive_v to the memory address of v (fact pointer)
    println!("slice_v is {:?}", slice_v);
    let slice_v: &[i32] = &v[2..3]; // point to elements 2 to 4, 4 not included, its a NON OWNING REFERENCE
    println!("slice_v is {:?}", slice_v);

    // they are refenreces to a pointer that points to an slice of valuyes of the original
    // WHEN you want to write a function that operates with an array or vector, we will use slices

}
