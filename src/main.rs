use std::io;

fn main() {
    println!("Guess the number");

    println!("Provide your guess");

    // declare a variable to hold users input
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line!");

    print!("You guessed: {}", guess)
}