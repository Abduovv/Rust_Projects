use rand::Rng;
use std::io;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!");

    // Generate secret number ONCE (outside the loop)
    let secret_number = rand::thread_rng().gen_range(1..101);
    // Uncomment the next line for debugging:
    // println!("Secret Number: {}", secret_number);

    let max_attempts = 5; // Maximum number of attempts
    let mut guesses = 0; // Counter for attempts

    loop {
        println!("Please input your guess (or type 'quit' to exit).");

        let mut guess = String::new();

        // Read input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim and convert to lowercase for "quit" check
        let guess = guess.trim().to_lowercase();

        // Allow the user to quit
        if guess == "quit" {
            println!("{}", "Goodbye!".yellow());
            break;
        }

        // Convert input to number (handle errors)
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid number!".red());
                continue;
            }
        };

        guesses += 1; // Increment guess counter
        println!("You guessed: {} (Attempt #{})", guess, guesses);

        // Compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break; // Exit the loop on correct guess
            }
            Ordering::Greater => println!("{}", "Too Big".red()),
        }

        // Check if the user has run out of attempts
        if guesses >= max_attempts {
            println!(
                "{}",
                format!("Game Over! The secret number was {}.", secret_number).red()
            );
            break;
        }
    }
}