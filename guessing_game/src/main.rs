extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing Game!");
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101); // hard lower bound, soft upper bound
        
        println!("Secret number was: {}", secret_number);
        println!("Give me a number:");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read the line.");

        let guess: u32 = guess.trim().parse()
            .expect("Couldn't convert to number.");

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Was Less!"),
            Ordering::Greater => println!("Was Greater!"),
            Ordering::Equal   => {
                println!("Was Equal! You won! Bye!");
                break;
            },
        }
        
        println!("You guessed: {}", guess);
    }
}