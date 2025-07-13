use text_colorizer::*;

#[derive(Debug)]
struct City {
    city: String,
    population: u64,
}

// code without closures
fn sort_population(city: &mut Vec::<City>) {
    city.sort_by_key(pop_helper)
}

fn pop_helper(pop: &City) -> u64 {
    pop.population
}

fn create_and_sort_no_closure(){
    println!("{} Struct Vector sort no closure", "Starting".red());
    let a = City{city: String::from("A"), population: 100};
    let b = City{city: String::from("B"), population: 500};
    let c = City{city: String::from("C"), population: 30};
    let d = City{city: String::from("D"), population: 890};
    let e = City{city: String::from("E"), population: 5};

    let mut vec: Vec::<City> = vec![a, b, c,];
    vec.push(d);
    vec.push(e);

    println!("Vec is {:?}", vec);

    sort_population(&mut vec);

    println!("Vec now is {:?}", vec);
    println!("{} Struct Vector sort no closure", "Finished".green());
}


/// CLOSURES

fn sort_pop_closure(pop: &mut Vec::<City>) {
    pop.sort_by_key(|p| p.population)  // the |p| is the argument in the closure
}


fn create_and_sort_with_closure(){
    println!("{} Struct Vector sort with closure", "Starting".red());
    let a = City{city: String::from("A"), population: 100};
    let b = City{city: String::from("B"), population: 500};
    let c = City{city: String::from("C"), population: 30};
    let d = City{city: String::from("D"), population: 890};
    let e = City{city: String::from("E"), population: 5};

    let mut vec: Vec::<City> = vec![a, b, c,];
    vec.push(d);
    vec.push(e);

    println!("Vec is {:?}", vec);

    sort_pop_closure(&mut vec);

    println!("Vec now is {:?}", vec);
    println!("{} Struct Vector sort with closure", "Finished".green());
}

fn main() {
    println!("{}", "Starting program...".yellow());
    // closures are anonymous functions, this is, functions without a name
    // they can be save inside a variable or pass as argument to another functions
    create_and_sort_no_closure();
    create_and_sort_with_closure();

}
