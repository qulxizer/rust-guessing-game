use std::io;
use rand::Rng;

fn main() {
    // Greeting
    println!("Welcome to Guess the number!");
    println!("please enter your guess: ");

    // Generating Secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret number is {secret_number}");

    // Storing Guess
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("Your guess is {guess}:");
    
}
