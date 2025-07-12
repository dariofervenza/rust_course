struct Point<T> {  // assign a generec of type T
    x: T,
    y: T,  // what ever value we pass to T will be the value in x and y (talking about types)
}
struct PointMultiple<T, U> {
    x: T,
    y: U,
}

fn main() {
    println!("Hello, world!");

    // create code that can operate on values of many different types --> generics

    // generics are abstract standards fron concrete types --> we used them when talking about options

    let point = Point{x: 5.0, y: 5.0};
    let point2 = Point{x: "x", y: "y"};

    // so <T> is a placeholder but what we can not do is mix types:
    // let wrong = Point{x: 5.0, y: "other type"};
    // but we can specify many types in struct definition:
    let point = PointMultiple{x: 5.0, y: "y"};

    // they help us to make the code more modular and handle different types


}
