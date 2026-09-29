// lesson3/src/main.rs

// Lesson 3: Decisions
//
// if / else is a fork in the road. The program checks something that
// is true or false and takes one path.
//
// Two differences from TypeScript:
//   1. No parentheses around the condition.
//      TypeScript: if (age >= 18)     Rust: if age >= 18
//   2. The condition must be a real bool (true or false). A number
//      like 5 is not treated as "truthy", so `if count` is an error.
//
// Big idea: a block { } can hand back a value.
// Whatever is on the LAST line with NO semicolon is the value.
// A semicolon on the last line throws the value away, and the block
// hands back nothing, written (). Nothing can't be printed, so
// println! would fail with an error.
// This only matters when something is waiting for the value, like
// `let total = { ... };`. On a normal line, the semicolon just ends
// the instruction.

fn main() {
    let age = 18;

    if age >= 18 {
        println!("You can vote");
    } else {
        println!("Too young to vote");
    }

    // else if lets you chain more than two paths.
    let temperature = 30;

    if temperature > 35 {
        println!("It's hot");
    } else if temperature > 20 {
        println!("It's warm");
    } else {
        println!("It's cold");
    }

    // A block does private work, then hands back its last line.
    // subtotal and tax are scratch paper that only exist inside the
    // block. Only the last line comes out.
    let price = 500;
    let quantity = 5;

    let total = {
        let subtotal = price * quantity;
        let tax = subtotal * 20 / 100;
        subtotal + tax // no semicolon, so this is the value handed out
    };
    println!("Total is {}", total);

    // if is also an expression, so each side is a block that can hand
    // back a value. This replaces the ternary (a ? b : c) in TypeScript.
    // Both sides must hand back the same kind of value.
    let status = if age >= 18 { "adult" } else { "minor" };
    println!("{}", status);

    // Guessing check
    let secret = 7;
    let guess = 7;

    if guess < secret {
        println!("too low");
    } else if guess == secret {
        println!("correct");
    } else {
        println!("too high")
    }

    let result = if guess == secret { "win" } else { "try again" };
    println!("{}", result);

    // Try it: change guess to 7, then 9, and watch each path run.
    // Try it: add a semicolon after `subtotal + tax` and read the error.
}