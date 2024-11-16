// Import necessary modules for input/output and random number generation
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

// Main function to run the Hangman game
fn main() {
    // List of words to choose from for the game
    let words = vec!["wistful", "eclipse", "czar", "bananza"];
    // Randomly select a secret word from the list
    let secret_word = words
        .choose(&mut thread_rng())
        .expect("Word list is empty")
        .to_string();

    // Vector to keep track of guessed letters
    let mut guessed_letters: Vec<char> = Vec::new();
    // Number of remaining attempts the player has
    let mut remaining_attempts = 8;

    // Welcome message
    println!("Welcome to Hangman!");

    // Main game loop
    loop {
        // Display the current state of the word
        display_word(&secret_word, &guessed_letters);

        // Check the game state (win, lose, or continue)
        match game_state(&secret_word, &guessed_letters, remaining_attempts) {
            GameState::Win => {
                // Player has guessed the word correctly
                println!(
                    "\nCongratulations! You've guessed the word: {}",
                    secret_word
                );
                break; // Exit the loop
            }
            GameState::Lose => {
                // Player has run out of attempts
                println!("\nGame Over! The word was: {}", secret_word);
                break; // Exit the loop
            }
            GameState::Continue => {
                // Prompt the player to enter a letter guess
                println!("\nEnter a letter to guess:");
                let guess = read_guess(); // Read the player's guess

                // Check if the letter has already been guessed
                if guessed_letters.contains(&guess) {
                    println!("You already guessed that letter.");
                } else if secret_word.contains(guess) {
                    // Correct guess
                    guessed_letters.push(guess); // Add the letter to guessed letters
                    println!("Good guess!");
                } else {
                    // Incorrect guess
                    guessed_letters.push(guess); // Add the letter to guessed letters
                    remaining_attempts -= 1; // Decrease the remaining attempts
                    println!(
                        "Wrong guess! You have {} attempts remaining.",
                        remaining_attempts
                    );
                }
            }
        }
    }
}

// Function to display the current state of the word with guessed letters
fn display_word(secret_word: &str, guessed_letters: &Vec<char>) {
    // Create a string representation of the word with underscores for unguessed letters
    let display: String = secret_word
        .chars()
        .map(|c| if guessed_letters.contains(&c) { c } else { '_' }) // Replace unguessed letters with '_'
        .collect();
    // Print the current display of the word
    println!("Word: {}", display);
}

// Function to read a letter guess from the player
fn read_guess() -> char {
    loop {
        let mut guess = String::new();
        io::stdout().flush().unwrap(); // Ensure prompt is displayed immediately
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Read input from the player

        // Get the first character of the input
        if let Some(c) = guess.trim().chars().next() {
            // Check if the character is an alphabetic letter
            if c.is_alphabetic() {
                return c.to_ascii_lowercase(); // Return the letter in lowercase
            } else {
                println!("Please enter a valid letter."); // Prompt for valid input
            }
        }
    }
}

// Enum to represent the possible states of the game
enum GameState {
    Win,      // Player has won
    Lose,     // Player has lost
    Continue, // Game is still ongoing
}

// Function to determine the current state of the game
fn game_state(
    secret_word: &str,
    guessed_letters: &Vec<char>,
    remaining_attempts: i32,
) -> GameState {
    // Check if all letters in the secret word have been guessed
    if secret_word.chars().all(|c| guessed_letters.contains(&c)) {
        GameState::Win // Return Win state
    } else if remaining_attempts <= 0 {
        GameState::Lose // Return Lose state
    } else {
        GameState::Continue // Return Continue state
    }
}
