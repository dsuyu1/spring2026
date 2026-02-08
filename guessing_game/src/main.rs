// implement the check_guess function
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0  // guess is correct
    }
    else if guess > secret {
        1  // guess is too high
    }
    else {
        -1  // guess is too low
    }
}

fn main() {
    // declare a mutable variable to store the secret number
    let secret: i32 = 42;
    
    // array of guesses to simulate user input
    // this is to meet the requirement from the assignment:
    // "Set a mutable guess variable to a number of your choice (simulating user input)"
    let guesses: [i32; 7] = [50, 25, 35, 40, 45, 43, 42];
    
    // track the number of guesses
    let mut guess_count: i32 = 0;
    
    // write an intro for the beginnning of the game
    println!("Welcome to the Guessing Game! :)");
    println!("Pick a number between 1 and 100\n");
    
    // use a loop to repeatedly make guesses
    let mut index: usize = 0;
    
    loop {
        // get the next guess from the array
        let guess: i32 = guesses[index];
        guess_count += 1;
        
        println!("Guess {}: {}", guess_count, guess);
        
        // call the check_guess function
        let result: i32 = check_guess(guess, secret);
        
        // use if-else expressions to print the feedback
        if result == 0 {
            println!("Correct! You found the secret number!");
            println!("It took you {} guesses.", guess_count);
            break;  // Exit the loop when guess is correct
        }
        else if result == 1 {
            println!("Too high! Try a lower number.\n");
        }
        else {
            println!("Too low! Try a higher number.\n");
        }
        
        index += 1;
    }
}
