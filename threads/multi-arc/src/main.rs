use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let timeout = 1;
    // Thread messages
    let messages = Arc::new(vec!["I'm the first thread!", "I'm the second thread!"]);

    // Thread vector, tracking all created threads
    let mut handles = Vec::new();

    for i in 0..messages.len() {
        let messages = Arc::clone(&messages);

        // Each thread is owning an reference counted messages vector
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs(timeout));
            println!("{}", messages[i]);
        });

        // Add the created thread to the thread vector.
        handles.push(handle);
    }

    // Now wait for all threads to terminate.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Messages:");
    for (i, m) in messages.iter().enumerate() {
        println!("\tMessage #{}: [{}]", i, m);
    }
}
