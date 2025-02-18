// Assignment 3: Guessing Game

// Function to check guess.
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

// Checking and counting guesses.
fn main() {
    let mut guesses = 0;
    let secret_number = 11;
    let mut guess = 25;

    loop {
        
        guesses += 1;
        
        let result = check_guess(guess, secret_number);

        if result == 0 {
            println!("Your guess of {} is correct!", guess);
            break;
        } else if result == 1 {
            println!("Your guess of {} is too high. Try again :)", guess);
            
        } else {
            println!("Your guess of {} is too low. Try again :)", guess);
            
        }

        if result == 1 {
            guess -= 3;
        } else {
            guess += 5;
        }

    }
    // Printing amount of guesses.
    println!("This is your amount of guesses: {}", guesses);
    
}