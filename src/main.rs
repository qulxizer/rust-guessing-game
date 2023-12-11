use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Greeting
    println!("Welcome to Guess the number!");
    
    // Generating Secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        // Storing Guess
        println!("please enter your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Try a bigger number!"),
            Ordering::Greater => println!("Try a smaller!"),
            Ordering::Equal => {
                println!("That's right!");
                break;
            }
    }
    };
    
}
