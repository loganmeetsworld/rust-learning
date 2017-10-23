use std::io;


fn main() {
    println!("Welcome to the guessing Game!");
    println!("Give me a number:");
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
        .expect("Failed to read the line.");
    
    println!("You guessed: {}", guess);
}