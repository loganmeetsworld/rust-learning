extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing Game!");
    loop {
        println!("Give me a number:");

        let secret_number = rand::thread_rng().gen_range(1, 101); // hard lower bound, soft upper bound
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read the line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error processing your input. Is it a number?");
                continue;
            },
        };
  

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Was Less!"),
            Ordering::Greater => println!("Was Greater!"),
            Ordering::Equal   => {
                println!("Was Equal! You won! Bye!");
                break;
            },
        }

        println!("Secret number was: {}", secret_number);
        println!("You guessed wrong, your guess: {}", guess);
    }
}