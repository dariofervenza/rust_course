struct Square {
    width: u32,
    height: u32
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn whats_my_width(&self) -> u32 {
        self.width
    }
}

struct SquareMut {
    width: u32,
    height: u32
}

impl SquareMut {
    fn change_width(&mut self, new_width: u32){
        self.width = new_width
    }
    fn whats_my_width(&self) -> u32 {
        self.width
    }
}

fn main() {
    println!("Hello, world!");

    // methods are similar to functions but defined in the context of a struct, enum or trade object

    // they have self as first parameter
    let sq = Square{width: 5, height: 5};
    println!("{}", sq.area());
    println!("{}", sq.whats_my_width());

    // what about a mutable struct?

    let mut sq =  SquareMut{width: 5, height: 5};
    sq.change_width(6);
    println!("{}", sq.whats_my_width());


}
