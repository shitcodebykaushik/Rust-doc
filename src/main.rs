use std::thread;

fn main() {
    // Spawn two threads to run tasks in parallel
    let thread1 = thread::spawn(|| {
        println!("Task 1 running in parallel");
    });

    let thread2 = thread::spawn(|| {
        println!("Task 2 running in parallel");
    });
    
    // Wait for threads to complete (if necessary)
    thread1.join().unwrap();
    thread2.join().unwrap();
}