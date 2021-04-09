use std::thread;
use std::time::Duration;

fn main() {
    let mut timeout = 500;
    let mut vector = vec!["hello"];
    println!("{:?}", vector);

    let handle = thread::spawn(move || {
        timeout = 5000;
        vector.push("world!");
        thread::sleep(Duration::from_millis(timeout));
        println!("Thread is done (timeout {} vector {:?})", timeout, vector);
    });

    println!("MAIN: Waiting for our spawned thread (timeout {})", timeout);
    handle.join().unwrap();
}
