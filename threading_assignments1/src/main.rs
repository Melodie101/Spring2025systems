use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};


// Assignment 1
fn main() {
    println!("Assignment 1:");
    println!("Main thread starting");
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 3 threads
    for i in 1..=3 {
        // TODO: Spawn a thread and store its handle
        let handle = thread::spawn(move || {
            // Simulate some work
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(500));
            println!("Thread {} finished", i);
        });
        
        // TODO: Store the handle

        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    
    println!("All threads completed.");




// Assignment 2
    // TODO: Create a shared counter using Arc and Mutex
    println!("\nAssignment 2:");
    let counter = Arc::new(Mutex::new(0));
    
    // TODO: Create a vector to store thread handles
    let mut handles = vec![];
    
    // TODO: Spawn 5 threads
    for _i in 1..=5 {
        // TODO: Clone the Arc for the thread
        let counter_clone = Arc::clone(&counter);
        
        // TODO: Spawn a thread that increments the counter 10 times
        let handle = thread::spawn(move || {
            // TODO: Increment counter 10 times
            for _i in 0..10 {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
            
        });
        
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // TODO: Print the final value of the counter
    println!("Final value of counter {}", *counter.lock().unwrap());


}