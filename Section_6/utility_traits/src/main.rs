struct Course {
    headline: String,
    author: String,
}

impl Drop for Course {
    fn drop(&mut self) {
        println!("Droping {}", self.author)
    }
}

// trait clone definition

trait Clone: Sized {
    // clone is very time and memory consuming
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self){
        *self = source.clone()
    }
}

// ways to implement a copy
#[derive(Copy, Clone)]
struct MyStructM;

// do it manually

struct My2Struct;

impl Copy for My2Struct { }
impl Clone for My2Struct {
    fn clone (&self) -> My2Struct {
        *self
    }
}

// NOTE: Implmenting copy adds extra restrictions to our type, so evaluate pros and cons before doing it

fn main() {
    println!("Hello, world!");
    // they are a grab bag of various trains from the std library hat have an important impact on how rust is written


    
    // DROP TRAIT: When we referenced the freeing of the resources  the value has using
    // happend when: a variable goes out of scope of an expression, removing elements from a vector, etc, ...

    let course1 = Course{headline: "aa".to_string(), author: String::from("Manolete")};
    drop(course1);
    let course1 = Course{headline: "aa".to_string(), author: String::from("Paquito")};
    // without calling the drop, it runs the drop itself at the end of the program




    // CLONE TRAIT: for types that can make copies of itself
    // so it needs to construct an independent copy using the self keyword
    // i32 and bool implement clone, but other things as mutex (we will see them) do not implement it




    // COPY TRAIT: simple types like i32 do not own any recources so they can be a copy type
    // a type if copy if it implements copy trait and then clone

    // eg:
    // pub trait Copy: Clone { }

    // rust only allows implementing copy if a shallow byte for byte copy is all it needs 
    // therefore, any other type tht owns any resources like a heat buffer ot operating system handles can not implement copy
    // SEE ABOVE TO CHECK EXAMPLES



    // FROM and INTO TRAITs: allows us to perform conversion on a value of one type and then return it as another

    // INTO TRAIT  SIGNATURE
    // fn into(self) -> T
    // FROM TRAIT SIGNATURE
    // fn from (T) -> self

    // Therefore, from will act as a constructor when producing an instance of a type from other single value



    // TryFrom and TryInto TRAITS
    
    // eg trying to implement an integer i32 from an integer i64 --> it will result in lost of information
    // tryform and tryinto are the error catchers

    // signatures:
    // fn try_from(value: T) -> Result<Selfm Self::Error> 

    // there fore they will give us the same output as the from and into, but the allows us to decide what to do in case of an error (exception)

    

} // here course1 goes out of scope so it auto runs drop(course1);
