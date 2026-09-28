// lesson4/src/main.rs

// Lesson 4: Loops
//
// A loop repeats instructions, like a washing machine cycle.
// Rust has three kinds, and they differ in how they decide to stop:
//   loop    repeats forever until you say break
//   while   repeats as long as a condition is true
//   for     repeats once for each item in a range or list
//
// A loop needs something that eventually makes it stop. Often that is
// a counter, so the counter must be an unlocked box (let mut).

fn main() {
    // loop: runs until break.
    let mut count = 0;
    loop {
        count = count + 1;
        println!("loop round {}", count);
        if count == 3 {
            break;
        }
    }

    // while: checks the condition before every round.
    // If nothing inside changes fuel, the loop never ends (an infinite
    // loop). Press Ctrl+C in the terminal to stop it.
    let mut fuel = 10;
    while fuel > 0 {
        println!("fuel: {}", fuel);
        fuel = fuel - 1;
    }
    println!("you are out of fuel");

    // for: once per item in a range.
    // 1..4 gives 1, 2, 3 and stops BEFORE 4.
    // 1..=4 gives 1, 2, 3, 4 (the = means "including the last one").
    // .rev() reads the range backwards.
    for i in (1..=5).rev() {
        println!("{}", i);
    }
    println!("liftoff"); // after the loop, so it prints only once

    // continue skips the rest of this round and jumps to the next one.
    for n in 1..=5 {
        if n == 3 {
            continue;
        }
        println!("{}", n); // prints 1, 2, 4, 5
    }

    // A loop is a block, so break can hand a value out of it
    // (same idea as Lesson 3, where a block hands back its last line).
    let mut tries = 0;
    let found = loop {
        tries = tries + 1;
        if tries * tries > 50 {
            break tries;
        }
    };
    println!("first number whose square passes 50: {}", found);

    // Multiplication table. Change number and every line follows,
    // because the text reads from the variable, not a typed 7.
    let number = 7;
    for i in 1..=10 {
        println!("{} * {} = {}", number, i, number * i);
    }

    // Try it: delete the `fuel = fuel - 1;` line above and run it.
    // The loop never ends. Press Ctrl+C to stop it.
}