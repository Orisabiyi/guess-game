use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    println!("Provide your guess");

    // generate random secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);

    // declare a variable to hold users input
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line!");

    // converting guess to a number from string
    let guess: u32 = guess.trim().parse().expect("Provide a number");

    print!("You guessed: {}, \n", guess);

    // compare guess with secret number using cmp and Ordering from standard library
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}