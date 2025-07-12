// import the binary heap into our scope

use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");

    // binary heaps are collectionss whose elements are kept loosely organized
    // there are 3 main methods in binary heaps that are push, pop and peek
    // BE AWARE: in bheaps, the highest value will be always at the fornt, but the rest wont be necessary ordered

    let mut bheap = BinaryHeap::new();
    bheap.push(1);
    bheap.push(15);
    bheap.push(162);
    bheap.push(56);
    bheap.push(95);

    println!("bheap is {:?}", bheap);

    // notice the order is not the same as we sent the elements
    // it puts first the highest elements because it indicates the priority
    // bheap is [162, 95, 15, 1, 56]

    bheap.pop();
    println!("bheap is {:?}", bheap);

    // PEEK METHOD: similar to pop but instead of removing the value, it leaves it there and also returns it
    println!("bheap.peek() is {:?}", bheap.peek());  // returns Some() or None
    println!("bheap after peek is {:?}", bheap);
    
    // It is useful to manage priorities, as we add values, the higher the priority , they will be at the front of the head, so they will be first pop or peek

    let sorted = bheap.clone().into_sorted_vec();
    println!("Sorted: {:?}", sorted);
}
