# CS333 Programming Languages - Rust vs C Presentation

This repository contains code examples accompanying the final presentation for CS333.
The examples demonstrate **four key features** of the Rust programming language and contrast them with C, highlighting Rust's focus on safety and modern design.

## Author
**Muneeb Azfar Nafees**  
Colby College  
Fall 2025

## Code Examples

### 1. Polymorphism: Traits vs Generics
- **File**: [`traits_generic.rs`](./traits_generic.rs)
- **Feature**: Demonstrates Rust's approach to polymorphism using **Traits**.
- **Comparison**: 
  - Shows both Static Dispatch (Generics) and Dynamic Dispatch (Trait Objects).
  - Contrasts with C's pattern of using **structs with function pointers** to emulate polymorphism, and shows how Rust enforces type safety at compile time.

### 2. Memory Management: Ownership & Borrowing
- **File**: [`memory_safety.rs`](./memory_safety.rs)
- **Feature**: Demonstrates Rust's **Ownership** and **Borrowing** rules.
- **Comparison**:
  - Shows how Rust manages memory without a Garbage Collector (like Java) or manual `malloc`/`free` (like C).
  - Illustrates protection against common C errors like "Double Free" or "Dangling Pointers" via the Borrow Checker.

### 3. Concurrency: Thread Safety
- **File**: [`concurrency.rs`](./concurrency.rs)
- **Feature**: Demonstrates "Fearless Concurrency" using `Arc` (Atomic Reference Counting) and `Mutex`.
- **Comparison**:
  - Shows how Rust prevents Data Races at compile time.
  - In C, multithreading requires careful manual synchronization (e.g., `pthreads`), which is prone to race conditions if mistakes are made. Rust refuses to compile code that shares mutable data across threads without safety mechanisms (like Mutex and Arc).

### 4. Error Handling: Result and Option
- **File**: [`error_handling.rs`](./error_handling.rs)
- **Feature**: Demonstrates typed error handling with `Result<T, E>` and `Option<T>`.
- **Comparison**:
  - Rust forces you to handle errors explicitly with matching.
  - In C, functions typically return special values or `NULL`, which are easy to ignore.

## How to Run
To run any of the examples, ensure you have Rust installed (`rustc`).

```bash
# Compile
rustc filename.rs

# Run
./filename
```

Example:
```bash
rustc concurrency.rs
./concurrency
```
