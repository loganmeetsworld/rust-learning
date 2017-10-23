extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing Game!");
    println!("Give me a number:");
    let secret_number = rand::thread_rng().gen_range(1, 101); // hard lower bound, soft upper bound
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
        .expect("Failed to read the line.");

    let guess: u32 = guess.trim().parse()
        .expect("Couldn't convert to number.");

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Was Less!"),
        Ordering::Greater => println!("Was Greater!"),
        Ordering::Equal   => println!("Was Equal!"),
    }
    
    println!("You guessed: {}", guess);
    println!("Secret number was: {}", secret_number);
}