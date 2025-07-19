trait Overview {
    fn overview(&self) -> String;
}

struct Course {
    headline: String,
    author: String,
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course{
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}
impl Overview for AnotherCourse{
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

trait OverviewDefault {
    fn overview_default(&self) -> String {
        String::from("Rust course")
    }
}

struct Course2 {
    headline: String,
    author: String,
}

impl OverviewDefault for Course2{
    fn overview_default(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}


// but we could have a default implmentation for Overvier

struct Course3 {
    headline: String,
    author: String,
}

impl OverviewDefault for Course3{}

fn main() {
    println!("Hello, world!");

    // a trait represents a capability, something that a type can do can be shared with other types
    let course1 = Course2{headline: String::from("Headline"), author: String::from("Paco")};

    println!("{}", course1.overview_default());

    let course3 = Course3{headline: String::from("Headline"), author: String::from("Paco")};
    println!("{}", course3.overview_default());

}
