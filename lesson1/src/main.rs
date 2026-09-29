// lesson1/src/main.rs

// Lesson 1: Variables
//
// A variable is a labeled box that holds a value.
// let makes the box and sticks a label on it.
//
// In Rust, boxes are LOCKED by default (the value cannot change).
// Add mut to unlock a box (short for "mutable", meaning changeable).
// This is the opposite of TypeScript, where let is unlocked and
// const is locked.
//
// Why lock by default? When a box is locked, you never have to wonder
// if it changed behind your back. The code shows which boxes can
// change, because only those have mut.

fn main() {
    // Locked boxes: these can be read, but never changed.
    let name = "Destiny";
    let city = "Port Harcourt";
    let project = "jack";

    // An unlocked box, because we plan to change it later.
    let age = 18;

    // Each {} is a slot, filled in order by the values after the string.
    println!("Hi, this is {}, he is {} years old. He lives in {}.", name, age, city);
    println!("He's going to work on a new project called {}", project);

    // No let here, so this is not a new box. It swaps what is inside
    // the existing age box. It only works because age has mut.
    let age = 19;
    println!("A year later he is {} years old", age);

    // Try it: remove the // below and run cargo run.
    // Rust refuses, because city is locked.
    // city = "Calabar";
}