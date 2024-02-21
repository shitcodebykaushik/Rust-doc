use std::sync::mpsc;
use std::thread;

fn main() {
    // Create a channel with a sender and receiver
    let (sender, receiver) = mpsc::channel();

    // Spawn a thread as the producer
    let producer = thread::spawn(move || {
        // Send messages to the receiver
        for i in 0..2 {
            sender.send(i).unwrap();
            println!("Sent: {}", i);
            // Introduce some delay for demonstration
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    // Receive messages in the main thread
    for received in receiver {
        println!("Received: {}", received);
    }

    // Wait for the producer thread to finish
    producer.join().unwrap();
}
