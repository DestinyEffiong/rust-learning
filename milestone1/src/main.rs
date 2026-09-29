// milestone1/src/main.rs

// Milestone 1: Number guessing game
//
// The computer picks a secret number from 1 to 100. The player types
// guesses until they get it right, or run out of 7 tries.
//
// New things used here, beyond Part 1:
//   - a borrowed crate (rand), added with `cargo add rand`
//   - reading what the player types (std::io)
//   - turning typed text into a number (.parse())

use std::io;

fn main() {
    let secret = rand::random_range(1..=100);
    println!("I am thinking of a number from 1 to 100.");
    let mut tries = 0;

    loop {
        println!("Your guess:");

        // String::new() makes an empty, growable text box.
        // Unlike &str (fixed text baked into the program), a String
        // can be written into, which read_line needs to do.
        let mut guess = String::new();

        // &mut guess lends the box to read_line so it can fill it.
        // read_line can fail (rare), so Rust forces us to handle that
        // possibility. expect() means "crash with this message if it
        // failed."
        io::stdin()
            .read_line(&mut guess)
            .expect("could not read input");

        // Shadowing: a new box, same label, now holding a number
        // instead of text. trim() removes the newline left by Enter.
        // parse() can fail too (e.g. the player typed a word), so it
        // also needs handling.
        let guess: u32 = guess.trim().parse().expect("please type a number");
        tries += 1;

        // Distance between guess and secret, without ever going
        // negative (u32 cannot hold a negative number, so we always
        // subtract the smaller from the bigger).
        let gap = if guess < secret {
            secret - guess
        } else {
            guess - secret
        };

        if gap <= 5 && guess != secret {
            println!("very close");
        }

        // Check for a win BEFORE checking the tries limit, otherwise
        // a correct final guess gets overridden by the "out of tries"
        // message before it ever gets seen.
        if guess < secret {
            println!("too low");
        } else if guess > secret {
            println!("too high");
        } else {
            println!("correct!");
            println!("number of tries: {}", tries);
            break;
        }

        if tries >= 7 {
            println!("You have tried too many times, the secret number is: {}", secret);
            break;
        }
    }
}