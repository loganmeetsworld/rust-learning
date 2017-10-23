fn main() {
    let mut x = 1;
    let mut done = false;

    while !done {
        if x == 101 {
            done = true;
        } else if x % 3 == 0 && x % 5 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x)
        }
        x += 1;
    }

}
