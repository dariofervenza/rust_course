enum Pet {Dog, Cat, Fish}
// we can implement methods in enums

impl Pet {
    fn what_am_i(self) -> &'static str {  // because if will be destroyed after the method execution, so we nned to specify it to life longer
        match self {
            Pet::Dog => "I am a dog",
            Pet::Cat => "I am a cat",
            Pet::Fish => "I am a fish",
        }
    }
}

enum IpAddKind{
    V4(String),  // assign a datatype to an enum
    V6,
}

struct IpAddress{
    kind: IpAddKind,
    address: String,
}


fn main() {
    println!("Hello, world!");

    // they allow you to define a type by enumerating its possible variants
    let dog = Pet::Dog;
    let cat = Pet::Cat;
    let fish = Pet::Fish;
    println!("{}", dog.what_am_i());
    println!("{}", cat.what_am_i());
    println!("{}", fish.what_am_i());

    let home = IpAddress{kind: IpAddKind::V6, address: "192.168.1.1".to_string()};
    println!("{}", home.address);
    let home = IpAddress{
        kind: IpAddKind::V4(String::from("im an ip")),
        address: "192.168.1.1".to_string()
    };
    // println!("{}", home.kind); cant print it

}
