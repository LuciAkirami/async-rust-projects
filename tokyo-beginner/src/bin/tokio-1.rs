use std::thread;
use std::time::Duration;

// task1() prints a message for every second
async fn task1() {
    for i in 1..=5 {
        println!("Task 1: Message {}", i);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

// task1() prints a message for every two seconds
async fn task2() {
    for i in 1..=5 {
        println!("Task 2: Message {}", i);
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}

#[tokio::main]
async fn main() {
    // Spawn two asynchronous tasks
    let task1_handle = tokio::spawn(task1());
    let task2_handle = tokio::spawn(task2());

    // You can execute code in the main function while tasks are running
    println!("Main: Executing other code while tasks are running");
    thread::sleep(Duration::from_secs(2));
    println!("Main: Execution inside Main Completed");

    // Await for both tasks to complete
    let _ = tokio::join!(task1_handle, task2_handle);
}
