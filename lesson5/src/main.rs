// Lesson 5: Functions
//
// A function is a named piece of code you can reuse. Think of a vending
// machine: you put things in (parameters), machinery does its job (the body),
// and something comes out (the return value).
//
// Build: the Lesson 2 calculator, with each operation pulled into its own function.

fn main() {
    // `main` is a function too. It is the one Rust runs first.
    // This function takes a value in and gives nothing back.
    greet("Destiny");

    // Each call below hands two numbers to a function and catches
    // the answer that comes back in a variable.
    // Note: 7.0 and 2.0 are decimals (f64). Rust will not accept 7 or 2 here.
    let sum = add(7.0, 2.0);
    let subtract = difference(7.0, 2.0);
    let multiply = product(7.0, 2.0);
    let divide = quotient(7.0, 2.0);
    let compare = compare(7.0, 2.0);

    // \n starts a new line, so each result prints on its own row
    println!(" 7 + 2 = {}\n 7 - 2 = {}\n 7 * 2 = {}\n 7 / 2 = {}\n 7 > 2 = {}", sum, subtract, multiply, divide, compare)
}

// `name: &str` is a slot that accepts text.
// There is no `->`, so this function gives nothing back, it only prints.
fn greet(name: &str) {
    println!("Hi, {}", name)
}

// Two slots (a and b), each accepting an f64 (a decimal number).
// `-> f64` promises that an f64 comes out.
fn add(a: f64, b: f64) -> f64 {
    // No semicolon on the last line, so this value is what the function hands back.
    // Adding a semicolon here would throw the value away and break the f64 promise.
    a + b
}

fn difference(a: f64, b: f64) -> f64 {
    a - b
}

fn product(a: f64, b: f64) -> f64 {
    a * b
}

fn quotient(a: f64, b: f64) -> f64 {
    a / b
}

// Functions can hand back other types too. This one returns true or false (a bool).
fn compare(a: f64, b: f64) -> bool {
    a > b
}