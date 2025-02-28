use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new(); // a mutable variable that holds a string value
        io::stdin().read_line(&mut guess).expect("Failed to read line "); // getting user input
        
        let guess: u32= guess.trim().parse().expect("Provide a number");
        
        println!("You guessed {} \n", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => print!("You won!"),
        }
    }
}