use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);  // number generator seeded by the OS

    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();  // mut -> mutable - changeable

    io::stdin()
        .read_line(&mut guess)      // &mut -> mutable reference to change contents of variable 
        .expect("Failed to read line"); // handles program when Result of read_line is 'Err'

    println!("You guessed : {guess}");
}
