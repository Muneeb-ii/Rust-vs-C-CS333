/**
 * This program demonstrates Rust's Memory Safety features: Ownership and Borrowing.
 * It shows how Rust manages memory automatically without a garbage collector or manual freeing.
 * 
 * Key Concepts:
 * 1. Ownership: Each value has a variable that's called its "owner". One owner at a time.
 * 2. Borrowing: References allow accessing data without taking ownership.
 * 3. Scope: When the owner goes out of scope, the value is dropped.
 * 
 * Comparison to C:
 * - No `malloc`/`free` needed (prevents memory leaks).
 * - Compiler prevents "Use After Free" errors (dangling pointers).
 * 
 * Muneeb Azfar Nafees 
 * Colby College
 * CS333 - Fall 2025
 * 
 * How to run:
 * rustc memory_safety.rs
 * ./memory_safety
 * 
 */

fn main() {
    // --- Ownership Example ---
    println!("--- Ownership Example ---");
    {
        let s1 = String::from("hello"); // s1 owns the string
        let s2 = s1; // Ownership moved to s2. s1 is now invalid.
        
        // println!("{}, world!", s1); // This would cause a compile-time error!
        println!("s1 was moved to s2. s2 says: {}", s2);
    } // s2 goes out of scope and memory is freed here.

    // --- Borrowing Example ---
    println!("\n--- Borrowing Example ---");
    let s3 = String::from("Rust");
    let len = calculate_length(&s3); // Pass a reference (borrow)
    
    // s3 is still valid because we only borrowed it
    println!("The length of '{}' is {}.", s3, len);

    // --- Mutable Borrowing ---
    let mut s4 = String::from("Hello");
    change(&mut s4);
    println!("\n--- Mutable Borrowing ---");
    println!("Modified string: {}", s4);
}

// Function borrowing a reference (read-only)
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Function taking a mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
