// // basic_concurrency_threads.rs
// use std::thread::{self, sleep};
// use std::time::Duration;

// fn main() {
//     // Spawn a new thread
//     let handle = thread::spawn(|| {
//         for i in 1..=5 {
//             println!("Spawned thread: {}", i);
//             thread::sleep(Duration::from_millis(500));
//         }
//     });

//     // Main thread work
//     for i in 1..=3 {
//         println!("Main thread: {}", i);
//         thread::sleep(Duration::from_millis(700));
//     }

//     // Wait for the spawned thread to finish
//     handle.join().unwrap();
// }

// // 1. Spawn a thread that prints numbers from 1 to 5 with a delay.
// // In the main thread, print "Main running" every second for 3 times. Wait for the thread to finish.
// use std::thread::{self, sleep};
// use std::time::Duration;
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..6 {
//             println!("spawn thread: {}", i);
//             sleep(Duration::from_millis(300));
//         }
//     });
//     for _ in 0..3 {
//         println!("Main running");
//         sleep(Duration::from_millis(500));
//     }
//     handle.join().unwrap();
// }
// // 2. Modify the thread to take ownership of a vector and print its contents.
// // Main thread should remain unaffected.
// use std::thread::{self, sleep};
// use std::time::Duration;
// fn main() {
//     let vec = vec![2, 3, 4, 6, 2, 1];
//     let handle = thread::spawn(move || {
//         for i in vec {
//             println!("{:?}", i);
//             sleep(Duration::from_millis(500));
//         }
//     });
//     handle.join().unwrap();
// }
// 3. Use `move` keyword with `thread::spawn` to capture and print a variable from main.
// Confirm what happens without `move`.
// use std::thread::{self};
// fn main(){
//     let n = "12".to_string();
//     println!("in main thread: {}",n);
//     let handler = thread::spawn(move||{
//         println!("Inside spawned thread: {}",n);

//     });
//     // println!("outside in main thread: {}",n); //Value moved, so error
//     handler.join().unwrap();
// }
// 4. Spawn two threads. One prints even numbers, another prints odd numbers. Main waits for both.
// use std::thread;
// fn main(){
//     let h1 = thread::spawn(||{
//         for i in 0..=20{
//             if i%2 == 0 { println!("Even: {}",i)}
//         }
//     });
//     let h2 = thread::spawn(||{
//         for i in 0..=20{
//             if i % 2 == 1 { println!("Odd: {}",i)}
//         }
//     });
//     h1.join().unwrap();
//     h2.join().unwrap();
// }
// 5. Create a thread that panics. Handle the panic using `join()` and match on the result.
use std::thread;
fn main(){
    let handler = thread::spawn(||{
        panic!("Panic called in Spawned thread");
    });
    match handler.join(){
        Ok(a) => println!("Fine: {}",a),
        Err(err) => println!("Error: {:?}",err),
    }
}
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
