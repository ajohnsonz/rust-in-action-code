use std::io;

fn main() {
    loop {
        // Create a new mutable String to store the user input
        let mut user_input = String::new();

        // Prompt the user for input
        println!("Please enter some text:");

        // Read the input from the user
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        // Print the user input back to the console
        println!("You entered: {}", user_input);
    }
}


// read the input (arabic, transliteration, english) from a file into an array

// choose a mode - show arabic and expect english, show english and expect transliteration, etc
// pick a random option from the array
// display the element from the array that matches the mode
// read the user input
// compare the user input with the expected answer based on the mode
// say correct or incorrect and give the expected answer
