// milestone1/src/main.rs
use std::io;

fn main() {
    let secret = rand::random_range(1..=100);
    println!("I am thinking of a number from 1 to 100.");

    loop {
        println!("Your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("could not read input");

        let guess: u32 = guess.trim().parse().expect("please type a number");

        if guess < secret {
            println!("too low");
        } else if guess > secret {
            println!("too high");
        } else {
            println!("correct!");
            break;
        }
    }
}