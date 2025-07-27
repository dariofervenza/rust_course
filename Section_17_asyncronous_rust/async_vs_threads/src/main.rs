use async_std::{ fs::File, io, prelude::*, task };

async fn read_file(path: &str) -> io::Result::<String> {
    let mut file = File::open(path).await?;  // it will wait until the request if finished, until the future produces its value
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

fn main() {
    println!("Hello, world!");
    // threads are suitable for an small number of tasks since they come with CPU and memory overhead
    // spawning and switching between threads is expensive and can consume resources even if the thread is idle
    // threads also allows to use sync code without huge changes

    // ASYNC provides significant reduced CPU and memory overhead --> specially with high input and ouput bound tasks such as databases and servers
    // it allows us to handle a much larger number of tasks than using threads
    // REASON: async takes advantage of a small number of expensive threads to handle a large amount of cheap tasks
    // it is not better than threads its just different


    // FUTURE TRAIT

    // a future is the heardt of async programming --> it is an asyncronous computation that can produce a value


    // ASYNC: import dependency in cargo toml (async-std = "1") --> crreated read_file function

    // TASKS:
    let tasks = task::spawn(async {  // it is necessary to have an async block in the task::spawn();
        let result = read_file("../read.txt").await;
        match result {
            Ok(v) => println!("File contains: {}", v),
            Err(e) => println!("Error while reading: {}", e),
        }
    });

    // differences in tasks: the TASK will be scheduled by the program, and if the task has to wait, the program is responsible for waking it back up

    println!("Task has started");
    task::block_on(tasks);
    println!("Stopped task");

    // tasks have the benefic that they are allocated in a single allocation, they have backchannel which allows them to propagate results and errors to the spawning task through the joint handle
    // they also carry useful metadata for debugging and the support task local storage

    // CONCLUSION: the task type is very similar to thread
}

// Future trait implementation

//trait Future {
//   type Output;
//    fn poll(self: Pin<&mut self>, cx: &mut Context) -> Poll<Self::Output>;  // poll is going to allows us to check on the state of the current computation
//
    // when the computation is done, poll will return poll::ready
    // if it is not done, poll will return poll::pending
    // it allows us to use a keyword called weight
//}
