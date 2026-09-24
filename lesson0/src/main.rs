// lesson0/src/main.rs

// Lesson 0: Workspace and first program
//
// Cargo is Rust's project manager (like npm in TypeScript).
// It creates the project, builds it, and runs it.
//
// Commands used in this lesson:
//   cargo new hello   creates a new project folder called "hello"
//   cargo check       proofreads the code for mistakes, builds nothing
//   cargo build       turns the code into a program file
//   cargo run         builds the program and then runs it
//
// Anything after // on a line is a comment. The computer ignores it.

// Every Rust program starts at a function called main.
// The curly braces { } wrap everything that belongs to it.
fn main() {
    // println! prints text to the terminal, then moves to a new line.
    // The ! means it is a macro: a shorthand that Rust expands into
    // more code before the program is built. Without the !, Rust looks
    // for a normal function called println and fails.
    println!("Hello");

    // {} is an empty slot. Rust fills the slots in order with the
    // values after the comma.
    println!("Hello {}", "Destiny");

    // Two slots, so two values. The text goes in quotes, numbers do not.
    println!("{} is {} years old", "Destiny", 18);

    // The semicolon ; means "this instruction is finished."
    // It is required when another line follows, and Rust gives an
    // error if it is missing. On the very last line of a block it is
    // optional, because the last line is the value the block hands back.
}