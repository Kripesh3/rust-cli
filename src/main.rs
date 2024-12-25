// A simple CLI in Rust
// This CLI will take a string as input from the user and return the string reversed

use std::env;

fn main() {
    // Collect the arguments in a vector
    let args: Vec<String> = env::args().collect();

    // Checking if the user has input a string bycargo run -- "your_input_string" checking the number of arguments
    if args.len() < 2 {
        println!("Please enter a string as a command-line argument.");
        return;
    }

    // Storing the string in a variable
    let input = args[1].clone();

    // Reversing the string
    let reversed_string: String = input.chars().rev().collect();

    // Printing the reversed string
    println!("The reversed string is: {}", reversed_string);
}

