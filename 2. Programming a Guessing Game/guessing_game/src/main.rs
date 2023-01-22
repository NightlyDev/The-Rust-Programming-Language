use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();  // mut -> mutable - changeable

    io::stdin()
        .read_line(&mut guess)      // &mut -> mutable reference to change contents of variable 
        .expect("Failed to read line"); // handles program when Result of read_line is 'Err'

    println!("You guessed : {guess}");
}
