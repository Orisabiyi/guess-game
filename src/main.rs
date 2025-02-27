use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Provide your guess");

    // generate random secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    // declare a variable to hold users input
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line!");

    print!("You guessed: {}", guess)
}