#[derive(Debug)]
struct Item {
    name: String
}

// create our own iterator

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32
}

impl Iterator for Range {
    type Item = u32;  // this is the return type
    fn next(&mut self) -> Option::<Self::Item> {
        if self.start >= self.end {
            return None;
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
}

fn check_inventory(items: Vec::<Item>, product: String) -> Vec::<Item> {
    // we use into_iter to create an iterable that will table ownership of the vector and call filter, to
    // adapt the iterator to another iterator (filter returns an iterator) with the elements that satisfy the closure
    // then call collect to gather the values into another vector
    items.into_iter().filter(|i| i.name == product).collect()  // will return a vector because of the collect
    // this is an example where a closure shines using iterators
}

fn main() {
    println!("Hello, world!");

    let vec = vec![1, 2, 3];
    for val in vec.iter() {
        println!("{}", val);
    }

    // all iterators implement a trait called Iterator ( which is defined in the std lib)
    //example
    let vec2 = vec![1, 2, 3];
    let mut iter = (&vec2).into_iter();  // we create a iterator here

    while let Some(v) = iter.next() {
        println!("{}", v);
    }

    // to clarify: an iterator is any type that implements the iterator trait and an iterable is any type that implement into trait

    let mut vec: Vec::<Item> = Vec::new();
    vec.push(Item{name: String::from("coat")});
    vec.push(Item{name: String::from("shirt")});
    vec.push(Item{name: String::from("shorts")});
    vec.push(Item{name: String::from("shoes")});

    let checked = check_inventory(vec, String::from("shirt"));
    println!("{:?}", checked);

    // trying out out custom made iterator
    let mut range = Range{start: 1, end: 10};
    for r in range {
        println!(" range vlue{}", r);
    }


    let mut range = Range{start: 1, end: 10};
    // when we transformed Range to an iterator, we now have also implemented many methods that come with Iterator like filter
    let vec: Vec::<u32> = range.filter(|x| x % 2 == 0).collect();
    println!("{:?}", vec);
}


// Iterator definition

//pub trait Iterator {
//    type Item;  // Item is the type of value the Iterator produces
//    fn next(&mut self) -> Option::<Self::Item>;  // the next either retirns a sum with the value in it or none to indicate the end of sequence
//}

// ALso there is the Intoiter iterator trait, the types that include it , its an iterator  because its something we can iterate over by calling to into method