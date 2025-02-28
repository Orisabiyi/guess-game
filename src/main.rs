use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new(); // a mutable variable that holds a string value
        io::stdin().read_line(&mut guess).expect("Failed to read line "); // getting user input

        if guess.trim() == "exit" {
            break;
        }
        
        let guess: u32=  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Provide a number, instead of letters or string");
                continue
            },
        };
        
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You won!".green());
                break;
            },
        }
    }
}