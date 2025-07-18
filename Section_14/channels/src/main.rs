use std::sync::mpsc;  // crate that provides channels
use std::thread;

fn main() {
    println!("Hello, world!");

    // message passing is where threads communicate by sending each other messages containing data
    // rust offers channels in its std lib

    // the channel has 2 ends, a transmitter and a receiver
    // the transmitter sends data and the receiver takes in the data
    // the channels is closed when the transmitter or the receiver end is droppeds (one of them)
    

    // EXAMPLE the transmitter will send a string to the receiver and it will print it out

    let (transmitter, receiver) = mpsc::channel();  // its a tuple

    let val = String::from("Transmitting");
    std::thread::spawn(move || {
        transmitter.send(val).unwrap();  // returns Result so we need to unwrap
    });

    let msg = receiver.recv().unwrap();  // also returns Result so we need to unwrap
    println!("Message received: {}", msg);

    // THEREFORE: MPSC is Multiproducer-single consumer --> its the description how channels work

    // how to create multiproducers?


    // FIRST: ownership rules and msg sending --> they help to write safe concurrent code

    // when we do transmitter.send(value) it takes ownership of the value  and when recv() receives it it takes the ownership then, obviously it is returner to the var that gets the value
    // rust does this for us so when we send a value, WE DO NOT reference it or use the initial var later on (val variable)

    // CREATE MULTIPLE PRODUCERS:

    let (transmitter, receiver) = mpsc::channel();
    // create another producer:
    let tx = transmitter.clone();
    std::thread::spawn(move || {
        let vec = vec![
            String::from("Transmitting"),
            String::from("From"),
            String::from("Original"),
            String::from("1111"),
            ];
        for val in vec {
            transmitter.send(val).unwrap();
        }
    });
    std::thread::spawn(move || {
        let vec = vec![
            String::from("Completely"),
            String::from("different"),
            String::from("tx"),
            String::from("22222"),
            ];
        for val in vec {
            tx.send(val).unwrap();
        }
    });

    while let msg_rcv = receiver.recv() {
        match msg_rcv {
            Ok(t) => println!("Got: {}", t),
            Err(_) => break,
        }
        
    }

    // it would be ok to do for rec in receiver{ println!("{}", rec)}


    // WHAT IF there are more messages sent that the receiver can get? The queue gets saturated
    // SOLUTION: Sync channel    
    let (transmitter, receiver) = mpsc::sync_channel(1000);  // works similar to the normal channel but we specify how many values the queu can hold so it is not wasting resources if we send more than we can get
    // if the queue is filled, the send method becomes a blocking operation, the thread freezes until the receiver freees some space in the queue

}


