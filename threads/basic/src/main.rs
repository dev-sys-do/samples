use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        println!("THREAD: Zzzzz....");
        thread::sleep(Duration::from_millis(500));
        println!("THREAD: Woke up");
    });

    println!("MAIN: Waiting for our spawned thread");
    handle.join().unwrap();
    println!("MAIN: Thread is done");
}
