use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // number generator seeded by the OS

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut -> mutable - changeable

        io::stdin()
            .read_line(&mut guess) // &mut -> mutable reference to change contents of variable
            .expect("Failed to read line"); // handles program when Result of read_line is 'Err'

        let guess: u32 = match guess.trim().parse() { // Shadowed a immutable variable from guess - used for conversion
            Ok(num) => num,     // if operation wents well
            Err(_) => continue, // if invalid characters have been put in -> next iteration of the loop
        };

        println!("You guessed : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
