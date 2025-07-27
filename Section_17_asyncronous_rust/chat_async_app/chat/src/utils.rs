// define reuslt and error types used in our app as weel as other functions
use async_std::prelude::*;  // set of convinient functions to work with strings and futures

use serde::de::DeserializeOwned;  // serde is a popular library for serializing and deserializing rust  data structures
// DeserializeOwned is a trait used for desirializing JSON data
// serde allows us to serialize to different formats such as JSON

use serde::Serialize;  // from in-memory representation to byte-string (or text format like JSON or yaml)

// this is the process to convert data between its in-memory representation and a byte-string that can transmitted or stored on disk

// the data can be any type such as a struct, a tuple, enum or even a primitive data type eg. i64

// DESERIALIZATION: convert it back to its original in memory representation

use std::error:Error;  // trait that represents errors
use std::marker::Unpin; // trait for types that can be safely Unpinned --> we will see it

// CREATE OUR CUSTOM TYPES

pub type ChatError = Box::<dyn Error + Send + Sync + 'static>;  // pointer to a dynamic error that implements Send and Sync traits and has a static lifetime
// the type keyword is used to define a type alias --> give an existing type name a new name that can be used interchangeably with the original type
// that is why we use the Box smart pointer (provides heap allocation and reference counting) -_> we use Box to provide a level of indirection and aovid large amounts of copying when dealing with complex data strcutures

// DYNAMIC error: is a trait that represents an error type that can be used within a result type --> defines the error types that can br thrown and caught by rust error handling system
// SEND and SYNC: rust marker traits that indicate that a type is safe to send on share between threads

pub type ChatResult<T> = Result::<T, ChatError>;  // our custom result

pub async fn send_json<O, P>(leaving: &mut O, packet: &P) -> ChatResult<()>
where
    O: async_std::io::Write + Unpin, // here we require that the leaving argument has a type that implements in the Write trait and implements the Unpin trait --> write allows to write bytes asyncronously while being safely unpinned 
    P: Serialize, // the packet argument has a type that implements the Serialize trait
{
    let mut json = serde_json::to_string(&packet)?;  // --> serialize the packet into a json string and return an error if it fails 
    json.push('\n');  // add a new line at the end
    leaving.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn receive<I, T>(incoming: I) -> impl Stream<Item = ChatResult::<T>>
where
    I: async_std::io::BufRead + Unpin,  // the generic type I has the trait that alllows us to async read and unpin it safely
    T: DeserializeOwned, // requiring the T geenric to have the DesirializedOwned trait --> desirialize without borrowing any data structure from the deserializer --> used for trait bounds on functions such as from_string()
{
    incoming.lines().map(|line| -> ChatResult<T> {  // lines method returns a stream over the lines of this byte stream on the incoming argument
        let li = line?;  // unwrap the line and return an error if fails
        let msg = serde_json::from_string::<T>(&li)?;  // deserialize the line to a chat message of type T and returning an errro if it fails
        Ok(msg)  // if the previous ? statements didnt failed, return Ok with the msg
    });
}