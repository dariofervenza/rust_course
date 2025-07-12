trait OverviewDefault {
    fn overview_default(&self) -> String {
        String::from("Rust course as param")
    }
}

struct Course {
    paco: String
}
impl OverviewDefault for Course{}


fn main() {
    println!("Hello, world!");

    // we can also pass traits as parameters
    let course = Course{paco: "paco".to_string()};
    call_overview(&course);
    call_overview2(&course);



}

fn call_overview(item: &impl OverviewDefault) {
    println!("{}", item.overview_default());
}

// we an simplyfy the function

fn call_overview2<T: OverviewDefault>(item: &T) {
    println!("{}", item.overview_default());
}

// fn overview4(item1: &impl OverviewDefault, item2: &impl OverviewDefault)  // this one can have 2 different types if we would want
// fn overview4<T: OverviewDefault>(item1: &T, item2: &T)  // this one can only has one type

// multiple trait bounces!!!!

// fn overview4(item1: &impl OverviewDefault + AnotherTrait)
// fn overview4<T: OverviewDefault + AnotherTrait>(item1: &T, item2: &T)