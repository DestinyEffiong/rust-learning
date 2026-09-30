// Lesson 6: Collections, part 1 (Vec)
//
// A Vec is a growable list of things that are all the same type.
// Think of a shopping list: the items are in order, you can add to
// the bottom, and each item has a position number starting at 0.

fn main() {
    // ---------- Part 1: the shopping list ----------

    // `vec!` makes a Vec with starting items. The `!` marks it as a macro,
    // a built-in tool that accepts any number of inputs.
    // `mut` is needed because the list will change when we push to it.
    let mut list = vec!["milk", "bread", "eggs"];

    // push adds an item to the end of the list
    list.push("rice");

    // `{}` can't print a Vec because Rust doesn't know which format you want.
    // `{:?}` prints everything inside it, for the programmer to read.
    // Try it: change `{:?}` to `{}` and read the compiler error.
    println!("{:?}", list);

    // Positions start at 0, so list[0] is the first item.
    // Try it: change 0 to 10. A Vec can grow, so its length is only known
    // while the program runs, and the crash happens at runtime (a panic).
    println!("first item: {}", list[0]);

    // len() is how many items are in the list
    println!("items: {}", list.len());

    // `&list` means "look at the list without taking it away"
    // (borrowing is covered properly in Lesson 10)
    for item in &list {
        println!("- {}", item);
    }

    // A plain array, like [70, 85, 90], has a fixed length.
    // Rust knows it before the program runs, so asking for position 10
    // is caught while compiling, not while running.

    // ---------- Part 2: Build, test scores ----------

    let mut scores = vec!(70, 85, 90);
    scores.push(100);

    // `total` collects the sum, `n` numbers each score (like a bank ticket machine)
    let mut total = 0;
    let mut n = 1;

    for i in &scores {
        total = i + total;
        println!(" {}. {}", n, i);
        n = n + 1;
    }

    println!("Total is: {}", total);
    println!("Number of scores is {}", scores.len());
}