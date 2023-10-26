/* Asynchronous File Operations using the Tokio Crate
The following operations can be performed asynchronously and concurrently and parallel
-> writing to a file
-> reading from a file
-> copying from one file to another
 */
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// fn to write to a file asynchronously
async fn write_to_file(num: usize) {
    let mut f = tokio::fs::File::create(format!("file{num}.txt"))
        .await
        .unwrap();
    let buffer = [97, 97, 97];
    for _ in 0..500_000 {
        f.write_all(&buffer).await.unwrap();
    }
}

// fn to read from a file asynchronously
async fn read_from_file(num: usize) {
    let mut f = tokio::fs::File::open(format!("file{num}.txt"))
        .await
        .unwrap();
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).await.unwrap();

    let buffer_len = buffer.len();
    println!("Total bytes read {buffer_len}");
}
#[tokio::main]
async fn main() {
    let file1 = write_to_file(1);
    let start = std::time::Instant::now();
    // awaiting on a single file operation asynchronously
    file1.await;
    let elasped = start.elapsed();
    println!(
        "Time taken for writing for single file {}",
        elasped.as_secs()
    );

    // -------- Writing to Multiple Files ------
    let file2 = write_to_file(2);
    let file3 = write_to_file(3);

    let start = std::time::Instant::now();
    // awaiting on mutiple file write operations asynchronously, and this takes the time as it takes to
    // execute a single file write operation as they run concurrently and parallel
    tokio::join!(file2, file3);
    let elasped = start.elapsed();
    println!(
        "Time taken for writing for multiple files {}",
        elasped.as_secs()
    );

    // Reading asynchronously from a file
    let read_file1 = read_from_file(1);
    let start = std::time::Instant::now();
    // awaiting on a single file read operation asynchronously
    read_file1.await;
    let elasped = start.elapsed();
    println!(
        "Time taken for reading a  single file {}",
        elasped.as_millis()
    );

    // ------- Reading from Multiple Files -------
    let read_file2 = read_from_file(2);
    let read_file3 = read_from_file(3);
    let read_file4 = read_from_file(1);
    let read_file5 = read_from_file(1);

    let start = std::time::Instant::now();
    // awaiting on mutiple file read operations asynchronously, and this takes the time as it takes to
    // execute a single file write operation as they run concurrently and parallel
    tokio::join!(read_file2, read_file3, read_file4, read_file5);
    let elasped = start.elapsed();
    println!(
        "Time taken for writing for multiple files {}",
        elasped.as_millis()
    );
}
