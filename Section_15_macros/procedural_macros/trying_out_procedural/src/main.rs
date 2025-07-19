use macros::debug_print;


#[debug_print]
fn main() {
    println!("Hello, world!");
    // in toml [dependencies] we point to our macros lib with the defined procedural macro
    // if we run this we should get the extrac print with the function name
}
