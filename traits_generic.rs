/**
 * This program is a simple example of traits and generics in Rust.
 * It defines a Shape trait with an area method, and two structs Circle and Rectangle.
 * It then implements the Shape trait for both structs, and defines two functions 
 * total_area and total_area_mixed that calculate the total area of a list of shapes.
 * 
 * 
 * Muneeb Azfar Nafees 
 * Colby College
 * CS333 - Fall 2025
 * 
 * How to run:
 * rustc traits_generic.rs
 * ./traits_generic 
 * 
 */

// Trait definition
trait Shape {
    fn area(&self) -> f64;
}

// Circle struct definition
struct Circle {
    radius: f64,
}

// Rectangle struct definition
struct Rectangle {
    width: f64,
    height: f64,
}

// Circle implementation of Shape trait
impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

// Rectangle implementation of Shape trait
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Static dispatch with generics
fn total_area<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// Dynamic dispatch with trait objects
fn total_area_mixed(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// Main function
fn main() {
    let circles = vec![
        Circle { radius: 1.0 },
        Circle { radius: 2.0 },
    ];
    println!("Total circle area: {}", total_area(&circles).round()); // rounded to nearest whole number

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle { width: 3.0, height: 4.0 }),
    ];
    println!("Total mixed area: {}", total_area_mixed(&shapes).round());
}