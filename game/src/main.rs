use std::collections::HashSet; // Import HashSet from the standard collections library
use std::io::{self, Write}; // Import I/O library for reading input and writing output

// Function to display the current state of the word being guessed
fn display_word(secret_word: &str, guessed_letters: &HashSet<char>) {
    // Create a string representation of the word with guessed letters revealed
    let display: String = secret_word
        .chars()
        .map(|c| if guessed_letters.contains(&c) { c } else { '_' }) // Replace unguessed letters with '_'
        .collect();
    println!("Word: {}", display); // Print the current display of the word
}

// Function to read a letter guess from the user
fn read_guess() -> char {
    loop {
        let mut guess = String::new(); // Create a new string to store the user's input
        io::stdout().flush().unwrap(); // Ensure any buffered output is flushed to the console
        io::stdin()
            .read_line(&mut guess) // Read a line of input from the user
            .expect("Failed to read line");

        // Get the first character of the trimmed input
        if let Some(c) = guess.trim().chars().next() {
            // Check if the character is alphabetic
            if c.is_alphabetic() {
                return c.to_ascii_lowercase(); // Return the lowercase version of the character
            } else {
                println!("Please enter a valid letter."); // Prompt for a valid letter if input is invalid
            }
        }
    }
}

fn main() {
    // The word to guess in the game
    let secret_word = "wistful";
    let mut guessed_letters = HashSet::new(); // A set to keep track of guessed letters
    let mut remaining_attempts = 8; // Number of attempts the player has before losing

    println!("Welcome to Hangman!"); // Welcome message

    // Main game loop that continues until the player runs out of attempts
    while remaining_attempts > 0 {
        display_word(&secret_word, &guessed_letters); // Display the current state of the word

        // Prompt the player to enter a letter to guess
        println!("\nEnter a letter to guess:");
        let guess = read_guess(); // Read the player's guess

        // Check if the guessed letter is in the secret word
        if secret_word.contains(guess) {
            guessed_letters.insert(guess); // Add the guessed letter to the set of guessed letters
            println!("Good guess!"); // Inform the player that the guess was correct

            // Check if the player has guessed all the letters in the secret word
            if secret_word.chars().all(|c| guessed_letters.contains(&c)) {
                println!(
                    "\nCongratulations! You've guessed the word: {}",
                    secret_word // Congratulate the player for winning
                );
                break; // Exit the game loop
            }
        } else {
            remaining_attempts -= 1; // Decrement the remaining attempts if the guess was wrong
            println!(
                "Wrong guess! You have {} attempts remaining.",
                remaining_attempts // Inform the player of remaining attempts
            );
        }
    }

    // If the player has run out of attempts, inform them that the game is over
    if remaining_attempts == 0 {
        println!("\nGame Over! The word was: {}", secret_word);
    }
}
