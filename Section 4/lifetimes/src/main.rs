fn main() {
    println!("Hello, world!");

    // every reference has a lifetime

    // most of the time lifetimes are implicit and inferred, they are used to prevent dangling references

    // lets set a quick dangling ref

    let r;

    {
        let x = 5;
        r = &x;
    }

    // println!("{}", r);  // x -> ^^ borrowed value does not live long enough

    // to set a lifetime: 'a

    // &i32
    // &'a i32
    // &'a mut i32

    // voy por 6:24


}

fn example_funct<'a>(x: &'a str) -> &'a str {  // function with a reference and a explicit lifetime --> the parameter x and the return string slice must life ass the lifetime of take a
    x
}
// the return parameter type must match at least one of the type of one parameter of the function!! 

// when do the lifetime starts? why are they in rust???

// rust developers detected that rust devs where using the same lifetime annotations constantly
// so the compiler was infering this annotations, it was using 3 rules to do it:
// 1. eaahc parameter that is a reference gets its own life time parameter--> if we have one parameter, we have one lifetime parameter
// but if we have 2 parameters we have 2 lifetime parameters ('a and 'b)
// 2. if we have one lifetime parameter that is assigned to the return value
// 3. if there are multiple input lifetime parameters, but one is a reference to the &self or &mut self
// the lifetime of the self is assigned to all output life parameters
// that is why lifetime annotations are not needed in methods, because they will always assigned  to the self lifetime

// important--> they dont have to be same tpye, but the output lifetime annotation must be assigned to at least one parameters lifetime
// so the return value will live at least as lnog as the parameter 

fn example2<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
    // x if we return the life time of 'b, we cant return x
    y  // so when the borrower checker does  not how to infer which lifetime return, we need explicit lifetimes
    // and we need the borrower checker to ensure that here is memory safety
}
