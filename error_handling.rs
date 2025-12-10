/**
 * This program demonstrates Rust's Error Handling using Result and Option.
 * It shows how Rust treats errors as values that must be handled explicitly.
 * 
 * Key Concepts:
 * 1. Result<T, E>: An enum that represents either success (`Ok(T)`) or failure (`Err(E)`).
 * 2. Option<T>: An enum that represents either a value (`Some(T)`) or nothing (`None`).
 * 3. Pattern Matching: Using `match` to handle both cases safely.
 * 
 * Comparison to C:
 * - In C, errors are often signaled by returning special values (like -1 or NULL).
 * - It is easy to ignore these return values in C, leading to crashes.
 * - Rust forces you to handle the `Result` or `Option`, reducing runtime errors.
 * 
 * Muneeb Azfar Nafees 
 * Colby College
 * CS333 - Fall 2025
 * 
 * How to run:
 * rustc error_handling.rs
 * ./error_handling
 * 
 */

fn main() {
    let numerator = 10.0;
    let denominators = vec![2.0, 0.0, 5.0];

    for &denom in &denominators {
        match divide(numerator, denom) {
            Ok(result) => println!("{} / {} = {}", numerator, denom, result),
            Err(e) => println!("Error dividing considering {} / {}: {}", numerator, denom, e),
        }
    }
}

// Function returning a Result: logic is explicit in the type signature
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(numerator / denominator)
}
