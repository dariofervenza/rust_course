// create greatest common denominator macro

macro_rules! gcd_macro {
    ($a: expr, $b: expr) => {  // arguments in macros are created with $ and expr is expression
        {
            let mut m = $b;
            let mut n = $a;
            while m != 0 {
                if m < n {
                    let t = m;
                    m = n;
                    n = t;
                }
                m = m %n;
            }
            n
        }
    };
}

fn main() {
    println!("Hello, world!");

    // macros are a way of writing code that wwrites other code (meta programming)

    println!("{}", gcd_macro!(19, 59));  // it injects the macro code in the binary compilation --> its code that creates other code and allows us to reuse it

}
