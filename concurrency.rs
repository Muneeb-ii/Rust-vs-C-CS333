/**
 * This program demonstrates Rust's "Fearless Concurrency".
 * It uses threads to increment a shared counter safely.
 * 
 * Key Concepts:
 * 1. Thread Safety: Rust guarantees memory safety across threads.
 * 2. Arc (Atomic Reference Counted): Allows shared ownership across threads.
 * 3. Mutex (Mutual Exclusion): Ensures only one thread accesses data at a time.
 * 
 * Comparison to C:
 * - C requires manual `pthread_create` and `pthread_mutex_lock`.
 * - In C, forgetting to lock leads to Data Races (undefined behavior).
 * - Rust refuses to compile if **mutable shared data** is accessed across threads without safety mechanisms (like Mutex/Arc).
 * 
 * Muneeb Azfar Nafees 
 * Colby College
 * CS333 - Fall 2025
 * 
 * How to run:
 * rustc concurrency.rs
 * ./concurrency
 * 
 */

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a shared counter protected by a Mutex and wrapped in Arc (for multiple owners)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    println!("Starting 4 threads to increment counter 1000 times each...");

    for i in 0..4 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                // Lock the mutex to get exclusive access to the data
                let mut num = counter.lock().unwrap();
                *num += 1;
            } // Mutex is automatically unlocked here when `num` goes out of scope (RAII)
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Result: {}", *counter.lock().unwrap());
    println!("Expected Result: 4000");
}
