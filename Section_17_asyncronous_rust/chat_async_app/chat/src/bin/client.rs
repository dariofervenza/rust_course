use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::{ task, io, net };
use std::sync::Arc;

use chat::utils:: { self, ChatResult };
use chat::{ Client, Server };


fn get_value(mut input: &str) -> Option<(&str, &str)> {  // returns an Option with a tuple or None
    // extract join from 'join CHAT' or post from 'post CHAT MESSAGE'  // the user send eg 'join chat_europe' or 'post chat_europe hi im am a message'
    input = input.trim_start();  // removee left whitespaces

    if input.is_empty(){
        return None;
    }
    match input.find(char::is_whitespace) {
        Some(whitespace) => Some((&input[0..whitespace], &input[whitespace..])),  // tuple with the strign from character 0 to whitespace position and whitespace position to the end
        None => Some((input, "")),
    }
}


fn parse_input(line: &str) -> Option<Client> {
    let (input, remainder) = get_value(line)?;  // unwrap with ?
    if input == "join" {
        let (chat, remainder) = get_value(remainder)?;
        if !remainder.trim_start().is_empty() {
            return None;  // debe estar vacio, con ! decimos NOT 
        }
        return Some(Client::Join { chat_name: Arc::new(chat.to_string()), });
    } else if input == "post" {
        let (chat, remainder) = get_value(remainder)?;
        let message = remainder.trim_start().to_string();
        return Some(Client::Post {
            chat_name: Arc::new(chat.to_string()),
            message: Arc::new(message),
        });
    } else {
        println!("Unrecognized input: {:?}", line);
        return None;
    }
}

// create the function that allows us send the messages to the server
async fn send(mut send: net::TcpStream) -> ChatResult<()> {
    println!("Options: \njoin CHAT\Npost CHAT MESSAGE");
    let mut options = io::BufReader::new(io::stdin()).lines();  // creates a new input buffer reader for reading lines fron stdio

    while let Some(option_result) = options.next().await { // loop throught the lines we got in the command line (user input)
        let opt = option_result?;  // unwrap
        let req = match parse_input(&opt) {
            Some(req) => req,
            None => continue,
        };
        utils::send_json(&mut send, &req).await?;
        send.flush().await?;  // write the buffer and make sure all the data is sent
    }
    Ok(()) // it is (()) because () is what was in out chatresult

}



// create function to receive messages from the server
async fn messages(server: net::TcpStream) -> ChatResult<()> {
    let buf = io::BufReader::new(server);
    let mut stream = utils::receive(buf);
    while let Some(msg) = stream.next().await {
        match msg? {
            Server::Message { chat_name, message } => {
                println!("Chat name: {}\nMessage: {}\n", chat_name, message);
            },
            Server::Error(message) => {
                println!("Error received: {}", message);
            }
        }
    }
    Ok(())
}


// main function that connects to the server and sends and receives messages
fn main() -> ChatResult<()> {
    let addr = std::env::args().nth(1).expect("Address:PORT");  // tell the server ip and port as arguments (when executing the app)
    task::block_on(async {  // block main thread here
        let socket = net::TcpStream::connect(addr).await?;  // connecting to the server using ip and port // similar to pythons socket.socket
        socket.set_nodelay(true)?;  // ensure segments are sent as soon as possible even if there is a small amount of data
        // if it is not set, the data will buffer until it accumulates a minimum quantity

        // create 2 new tasks with the functions we created
        let send = send(socket.clone());
        let replies = messages(socket);

        // run these 2 new tasks concurrently and see which one completes first

        replies.race(send).await?;
        Ok(())
    });
}