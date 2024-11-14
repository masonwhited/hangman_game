use std::collections::HashSet;
use std::io::{self, Write};

fn display_word(secret_word: &str, guessed_letters: &HashSet<char>) {
    let display: String = secret_word
        .chars()
        .map(|c| if guessed_letters.contains(&c) { c } else { '_' })
        .collect();
    println!("Word: {}", display);
}

fn read_guess() -> char {
    loop {
        let mut guess = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if let Some(c) = guess.trim().chars().next() {
            if c.is_alphabetic() {
                return c.to_ascii_lowercase();
            } else {
                println!("Please enter a valid letter.");
            }
        }
    }
}

fn main() {
    // Word to guess
    let secret_word = "rustacean";
    let mut guessed_letters = HashSet::new();
    let mut remaining_attempts = 8;

    println!("Welcome to Hangman!");

    // Main game loop
    while remaining_attempts > 0 {
        display_word(&secret_word, &guessed_letters);

        // Get a guess from the player
        println!("\nEnter a letter to guess:");
        let guess = read_guess();

        if secret_word.contains(guess) {
            guessed_letters.insert(guess);
            println!("Good guess!");

            // Check if the player has won
            if secret_word.chars().all(|c| guessed_letters.contains(&c)) {
                println!(
                    "\nCongratulations! You've guessed the word: {}",
                    secret_word
                );
                break;
            }
        } else {
            remaining_attempts -= 1;
            println!(
                "Wrong guess! You have {} attempts remaining.",
                remaining_attempts
            );
        }
    }

    if remaining_attempts == 0 {
        println!("\nGame Over! The word was: {}", secret_word);
    }
}
