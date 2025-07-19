
// it will calculate the average of the numbers we pass in

macro_rules! average {
    //  if the list is empty
    ( $(,)*) => {{  // is a regex it is used to match 0 or more (because of *) comma separated values --> this is used often in macro definitions that expect a list of items
        0.0
    }};

    // recursive case to calculate average of a non empty list
    ( $($val: expr), + $(,)*) => {{  // matches values in the val param that are separated by commas
        let count = 0usize $(+ { let _ = stringify!($val); 1})*;
        // the '$(+ { let _ = stringify!($val); 1})*' lets us count the values in the list
        // for each value in list rust will expand the expression and add 1 to the count
        // so now we have the denominator of the average

        // sum of the items
        let sum = 0.0 $(+ $val as f64)*;
        sum / count as f64
    }};
}

fn main() {
    println!("Hello, world!");
    // they are used to match a given pattern and replace it with the specified code
    // they can be used to define new control structures or simplify repetitive code
    println!("Average of nothing: {}", average!());
    println!("Average of floats: {}", average!(1.0, 2.0, 3.0));
    println!("Average of integers: {}", average!(1, 2, 3, 4, 5));


    // what is the difference between a declarative macro and a function like macro like we did in the macros project?  // macro_rules! gcd_macro { ($a: expr, $b: expr) => { { let mut m = $b;

    // a DECLARATIVE macro used a set of rules to match and transform input tokens into output tokens --> similar to a regular expression for replacing patterns
    // they are evaluated at compile time

    // a FUNCTION LIKE macro operates like a regular function, it accepts input arguments and outputs tokens
    // they are evaluated at compile time, same as declarative, but they have a more flexible syntax --> more complex transformations


    // declarative are simpler to define and use but are more limited in their capabilities
}
