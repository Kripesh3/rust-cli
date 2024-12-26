use rand::Rng;
use std::env;
use std::io;

fn main() {
    // Collect the arguments in a vector
    let args: Vec<String> = env::args().collect();

    // Checking if the user has input a string by checking the number of arguments
    if args.len() < 2 {
        println!("Please enter a string as a command-line argument.");
        return;
    }

    // Storing the string in a variable
    let input = args[1].clone();

    println!("Choose an option:");
    println!("1: Reverse the string");
    println!("2: UPPERCASE the string");
    println!("3: lowercase the string");
    println!("4: Generate a random password");

    // Read user input
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");


    match choice.trim().parse::<u32>() {
        Ok(1) => {
            // Reverse the string
            let reversed_string: String = input.chars().rev().collect();
            println!("The reversed string is: {}", reversed_string);
        }
        Ok(2) => {
            // UPPERCASE the string
            let uppercased_string: String = input.to_uppercase();
            println!("The uppercased string is: {}", uppercased_string);
        }
        Ok(3) => {
            // lowercase the string
            let lowercased_string: String = input.to_lowercase();
            println!("The lowercased string is: {}", lowercased_string);
        }
        Ok(4) => {
            // Generate a random password
            let mut rng = rand::thread_rng();
            let random_numbers: String = (0..3)
                .map(|_| rng.gen_range(0..10).to_string())
                .collect(); // Generate 3 random digits
            let password = format!("@{}{}", input, random_numbers);
            println!(" The generated password from the given string is: {}", password);
        }
        _ => {
            // Handle invalid input
            println!("Invalid choice.");
        }
    }
}
