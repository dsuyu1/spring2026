// is_even function
fn is_even(n: i32) -> bool {
    n % 2 == 0  // returns true if the number is divisible by 2 because remainder is 0
}

fn main() {
    // create an array of 10 integer numbers
    let numbers: [i32; 10] = [12, 15, 8, 21, 25, 7, 30, 9, 18, 4];
    
    // use a for loop to iterate and analyze each number
    for i in 0..10 {
        let number = numbers[i];
        
        // check if divisible by both 3 and 5
        if number % 15 == 0 {
            println!("{}: FizzBuzz", number);
        }
        // check if divisible by 3
        else if number % 3 == 0 {
            println!("{}: Fizz", number);
        }
        // check if divisible by 5
        else if number % 5 == 0 {
            println!("{}: Buzz", number);
        }
        // print if it's even or odd
        else if is_even(number) {
            println!("{}: Even", number);
        }
        else {
            println!("{}: Odd", number);
        }
    }
    
    // use a while loop to calculate the sum
    let mut sum: i32 = 0;
    let mut index: usize = 0;
    
    while index < 10 {
        sum += numbers[index];
        index += 1;
    }
    
    println!("Sum of all numbers: {}", sum);

    // use a loop to find the largest number
    let mut max: i32 = numbers[0];  // Start with the first number
    
    for i in 0..10 {
        let number = numbers[i];
        if number > max {
            max = number;
        }
    }
    
    println!("Largest number in the array: {}", max);
}

