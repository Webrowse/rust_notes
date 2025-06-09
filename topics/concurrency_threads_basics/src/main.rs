// basic_concurrency_threads.rs

use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Main thread work
    for i in 1..=3 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(700));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}

// 1. Spawn a thread that prints numbers from 1 to 5 with a delay. 
// In the main thread, print "Main running" every second for 3 times. Wait for the thread to finish.

// 2. Modify the thread to take ownership of a vector and print its contents. 
// Main thread should remain unaffected.

// 3. Use `move` keyword with `thread::spawn` to capture and print a variable from main. 
// Confirm what happens without `move`.

// 4. Spawn two threads. One prints even numbers, another prints odd numbers. Main waits for both.

// 5. Create a thread that panics. Handle the panic using `join()` and match on the result.

// 6. Add a long sleep in the main thread and let the spawned thread finish earlier. 
// Observe execution order.

// 7. Spawn a thread from inside another thread. 
// Ensure both child and grandchild threads complete before program exits.

// 8. Start five threads in a loop. Each should print its thread index. 
// Explain what happens if the loop variable is not moved or cloned.

// 9. Share a counter between threads using `Arc<Mutex<i32>>`. 
// Each thread increments it. Join all and print final value.

// 10. Modify the original example to spawn multiple threads printing in parallel. 
// Ensure main waits for all. Use a vector of handles.
