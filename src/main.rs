use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);

    println!("Weight on Mars: {} kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

// Ownership rules in Rust

// 1. Each value in Rust is owned by a variable.

// 2. When the owner/variable goes out of scope, its value is deallocated.

// 3. There can only be ONE owner at a time.
