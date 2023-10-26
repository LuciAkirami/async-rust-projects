async fn do_task(num: usize) {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("{num} task completed")
}

fn blocking_task() {
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("Blocking task completed");
}
#[tokio::main]
async fn main() {
    let task1 = do_task(1);
    let task2 = do_task(2);
    let task3 = do_task(3);

    // select! - Waits on multiple concurrent branches, returning when the first branch completes,
    // cancelling the remaining branches.
    tokio::select! {
        _ = task1 => {
            println!("task 1 selected");
        }
        _ = task2 => {
            println!("task 2 selected");
        }
        _ = task3 => {
            println!("task 3 selected");
        }
    }

    let task4 = do_task(4);
    let task5 = do_task(5);

    // Below will take 6 seconds to complete
    // task4 is .awaited, hence 2 seconds pass here
    task4.await;
    // a blocking task appears in between task4 and task5, hence task5 cannot run until blocking_task() is finished
    // this blocking task will block the entire thread by not yielding, hence preventing the executor from driving
    // other futures forward
    blocking_task();
    // task5 is now .awaited and will complete in 2 seconds
    task5.await;

    // To solve the above issue, we can move blocking_tasks, i.e. tasks that are CPU intensive and not async to
    // tokio::task::spawn_blocking, this will make the blocking_task to run on a separate thread

    let task6 = do_task(6);
    let task7 = do_task(7);

    // Below will take 4 seconds to complete
    // task6 is .awaited, hence 2 seconds pass here
    task6.await;
    // blocking_task is spawned in another thread, hence will not block the current thread it was spawned from
    tokio::task::spawn_blocking(blocking_task);
    // task7 will be .awaited as soon as task6 is completed
    task7.await;
}
