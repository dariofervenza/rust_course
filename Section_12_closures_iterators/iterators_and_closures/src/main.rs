fn main() {
    println!("Hello, world!");

    // we have to see 3 new traits: Fn, FnMut and FnOnce

    // Fn: family of closure and functs that you can call miltiple times without restrictions and borrows vlue from the environment inmutably

    // it includes all fn my-funct()

    // FnMut: is the family of closures that can be called multiple times if closure is itself declared mutable and inmutable borrows values

    // FnOnce is the family of closures that can be called once --> if the caller owns the closure and then the once part of the name is because the closure cannot take ownership of the same variables more than once

    // there fore, every Fn meets the requirements for an FnMute and every FnMute meets the reqs for FnOnce (one is subtrait of another)

    // so Fn is the most exclusive and most powerfull trait of the tree

    // examples:
    // || drop(v) -> takes no args and drops a val   ---->> this is FnOnce (can only be dropped once)
    // |args| v.contains(arg)  --> one param and checks if the vector contains the param  ---->> is Fn because we only check (nothing has to be mutable)
    // |args| v.push(arg) --> one param and pushes the arg to the vector ---->> FnMut because we modify the vector

    // COPY AND CLONE WITH  CLOSURES --> the rules are the same as we  saw (the do not hold any extra resource) and the compiler with infer which ones can implment copy

    let y = 5;
    let add_y = |x| x + y;
    let copy = add_y;  // this is closere being copied
    // check of the copy is correct
    println!("{}", add_y(copy(10)));  // it works

    // example where netihercopy nor clone works
    let mut  y = 5;
    let mut add_y = |x| { y +=x; y };  // sum y +x and return y
    let mut copy = add_y;   
    println!("{}", add_y(copy(10)));  // it does not work anymore because mutable values do not implement copy or clone

}
