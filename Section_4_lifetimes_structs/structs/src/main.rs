struct User {
    active: bool,
    username: String,
    sign_in_count: u32,

}

// tuple like:

struct Coordinates (i32, i32, i32);

// unitlike

struct UnitStruct;


fn main() {
    println!("Hello, world!");

    // name and pack similar value so you can deal with them as a single unit

    // rust has 3 types of structs: a named field, a tuple like and unit like

    // name field: gives a name to each component

    // a tuple like identified them by they order they appear

    // a unit like has no components at all
    let user1 = User{active: true, username: String::from("paco"), sign_in_count: 0};

    // access info in a struct: dot notation

    println!("{}", user1.username);

    let user2 = build_user("manolo".to_string());
    println!("{}", user2.username);

    // tuple like creation and access

    let coord = Coordinates(1, 2, 3);
    println!("{}", coord.0);

    // unit like

    // when we did 1..5  the .. means Range {start: 1, end: 5};

    // they will make sense when we will see traits

    // now we will see how to implement methods on these structs



}

fn build_user(username: String) -> User {
    User {
        username, // because they are called both username, it automatically maps username: username
        active: true,
        sign_in_count: 1,
    }
}
