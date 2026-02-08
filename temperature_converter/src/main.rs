// Step 1: Declare a constant for the freezing point of water in Fahrenheit
const FREEZING_POINT_F: f64 = 32.0;

// Step 2: Implement the conversion functions

// Function to convert Fahrenheit to Celsius
// Formula: C = (F - 32) × 5/9
// use -> arrow to indicate we're returning a 64 bit floating point number
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0 // divide the result by 9.0 to get the final Celsius value
}

// Function to convert Celsius to Fahrenheit
// Formula: F = C × 9/5 + 32
fn celsius_to_fahrenheit(c: f64) -> f64 { // this function never gets used but its required for the assignment
    c * 9.0 / 5.0 + FREEZING_POINT_F // this gets returned
}

fn main() {
    // Step 3: Declare a mutable variable with a starting temperature in Fahrenheit
    let mut fahrenheit = 32.0;
    
    // Convert the starting temperature to Celsius and print it
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}°F = {:.2}°C", fahrenheit, celsius);
    
    // Step 4: Loop to convert and print the next 5 temperatures
    // we need to convert 5 more temperatures (33°F through 37°F)
    for _ in 1..=5 {
        fahrenheit += 1.0;  // Increment the temperature by 1°F
        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("{}°F = {:.2}°C", fahrenheit, celsius);
    }
}
