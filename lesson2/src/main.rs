// lesson2/src/main.rs

// Lesson 2: Types
//
// Every box (variable) holds a specific kind of value, and Rust wants
// to know which kind. You can add two numbers, but you can't add a
// number to a word, the same way you can't pour water into an egg tray.
//
// Common types:
//   i32    whole number, can be negative      (42, -7)
//   u32    whole number, cannot be negative    (0, 250)
//   f64    number with a decimal point         (3.14)
//   bool   true or false
//   char   a single character, single quotes   ('D')
//   &str   text, double quotes                 ("Rust")
//
// A number with no dot (7) defaults to i32.
// A number with a dot (7.0) is f64.
// This matters for division: 7 / 2 with two i32 gives 3, because a
// whole number box cannot hold .5, so the remainder is dropped.
// 7.0 / 2.0 gives 3.5.

fn main() {
    // Shadowing: each `let` makes a NEW box with the same label.
    // This is different from `mut`, which reuses one box and can only
    // ever hold one kind of value.
    let score = 10;
    let score = score + 5; // new box, still a number
    println!("{}", score);

    let score = "fifteen"; // another new box, now holds text instead
    println!("{}", score);

    // Writing the type after : is optional, Rust can usually work it
    // out, but spelling it out here makes each kind clear.
    let whole: i32 = 42;
    let decimal: f64 = 3.14;
    let is_ready: bool = true;
    let letter: char = 'D';
    let word: &str = "Rust";
    println!("{}, {}, {}, {}, {}", whole, decimal, is_ready, letter, word);

    // Rust won't silently mix an i32 and an f64. `as` converts one
    // type into another on purpose.
    let total = whole as f64 + decimal;
    println!("total: {}", total);

    // Simple calculator
    let a: f64 = 7.0;
    let b: f64 = 2.0;
    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;
    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("{}", a > b); // comparisons produce a bool

    // Try it: change a and b above to whole numbers (7 and 2, no .0)
    // and watch quotient print 3 instead of 3.5.
}