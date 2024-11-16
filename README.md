## Overview

**Project Title**: Hangman Game

**Project Description**: a Terminal Hangman game using the Rust Programming language.

**Project Goals**: Randomly choose from a list of words to play hangman with

## Instructions for Build and Use

Steps to build and/or run the software:

1. Import random and I/O functionality
2. In main() make two lists (one for words, the other for the hangman graphics)
3. make a variable secret_word to randomly choose a word in the words list
4. make a variable guessed_letters to store the user's guesses in a list
5. make a variable for remaining attempts
6. make a variable for the current state of the hangman drawing
7. loop until the user guesses the word or runs out of attempts
8. in each iteration, print the current state of the hangman drawing and the word with underscores
9. ask the user for a guess
10. check if the guess is in the word, if so, replace the corresponding underscores with the guess
11. if the guess is not in the word, decrement the remaining attempts
12. if the user guesses the word, print a win message and exit the loop
13. if the user runs out of attempts, print a loss message and exit the loop
14. write a display_word function for displaying graphic and current state of the word
15. write a read_guess function to make sure the inputted guess is a valid letter of the alphabet

Instructions for using the software:

1. Open up the terminal
2. Enter the command `cd match_game`
3. Enter the command `cargo run`
4. Follow the prompts to play the game

## Development Environment 

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* VSCode

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [Rust Language](https://www.rust-lang.org/learn)
* [Rust Workshop Video](https://video.byui.edu/media/t/1_p8vj9m0w)
* [ChatGPT](https://www.chatgpt.com)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] I do not plan on working on this project in the future
