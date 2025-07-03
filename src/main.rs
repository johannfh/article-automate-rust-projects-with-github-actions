mod tests;

use std::io;

/// Calculates the area of a triangle given its base and height.
/// May also return negative values, if the input is negative.
pub fn calculate_triangle_area(base: f64, height: f64) -> f64 {
    0.5 * base * height
}

fn main() {
    println!("Hello, world!");

    // --- Read Base from User ---
    let mut base_input = String::new();
    println!("Please enter the base of the triangle:");
    io::stdin()
        // Read a line from stdin into base_input
        .read_line(&mut base_input)
        // Handle potential errors during reading
        .expect("Failed to read line for base.");

    let base: f64 = base_input
        .trim() // Trim leading and trailing whitespace
        .parse() // Try to parse the string to f64
        // Handle parsing errors
        .expect("Please enter a valid decimal number for the base.");

    if base < 0.0 {
        eprintln!("The base must not be negative.");
        return;
    }

    // --- Read Height from User ---
    let mut height_input = String::new();
    println!("Please enter the height of the triangle:");
    io::stdin()
        // Read a line from stdin into height_input
        .read_line(&mut height_input)
        // Handle potential errors during reading
        .expect("Failed to read line for height.");

    let height: f64 = height_input
        .trim() // Trim leading and trailing whitespace
        .parse() // Try to parse the string to f64
        // Handle parsing errors
        .expect("Please enter a valid decimal number for the height.");

    if height < 0.0 {
        eprintln!("The height must not be negative.");
        return;
    }

    // --- Calculate and Print Area ---
    let area = calculate_triangle_area(base, height);
    println!(
        "\nThe area of a triangle with base {:.2} and height {:.2} is: {:.2}",
        base, height, area
    );
}
