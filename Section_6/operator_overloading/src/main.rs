use std::ops::Add;

#[derive(Debug)]  // to use the {:?} debug formatter
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T>
    where
    T: Add<Output = T> {  // this restricts T for types that can be added to themselfs and yield another T value
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Point {
                x: self.x + rhs.x,  // rhs: right hand self
                y: self.y + rhs.y
            }
        }
    }

fn main() {
    println!("Hello, world!");

    // it allows us to do our own type support arithmetic, like when implementing __add__ in python

    let coord = Point{x: 5.0, y: 5.0};
    let coord2 = Point{x: 0.5, y: 2.0};
    let sum = coord + coord2;
    println!("{:?}", sum)
}
