// From: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // read_line returns Result type
        // If Result is Ok(value), the expect method returns value.
        // If Result is Err(err), the expect method panics with the error message provided.
        // expect method also works for Option types.
        // If Option is Some(value) then expect method would return the value.
        // If Option is None, then expect methig panics with the error message provided.
        // expect method provides aa convininet way to handle expected results and options
        // while also handling unexpected cases by panicking with a helpful error message.
        // expect is used in development or debugging process to catch and handle potentail errors
        // early on. However, in prod code, it's genrally recommended to use more robust error
        // handling like match or unwrap_or_elese to handle potential errors in controlled manner.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
