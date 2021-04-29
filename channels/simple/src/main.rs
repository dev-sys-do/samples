use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (sender, receiver) = channel();

    thread::spawn(move || {
        sender.send("Hello!").unwrap();
        sender.send("World!").unwrap();
    });

    for received in receiver {
        println!("Received: {}", received);
    }
}
