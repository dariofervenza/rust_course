use serde::{ Deserialize, Serialize };
use std::sync::Arc;
pub mod utils;

#[derive(Debug, Deserialize, Serialize, PartialEq)]  // PartialEq is used to define partial equality between 2 values of the same type --> used then to implement comparison == or !=
pub enum Client {
    Join {
        chat_name: Arc<String>,
    },
    Post {
        chat_name: Arc<String>,
        message: Arc<String>,
    }
}


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Server {
    Message {
        chat_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String)
}
