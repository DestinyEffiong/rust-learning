// Lesson 6: Collections (Vec and HashMap)
//
// A collection holds many values in one variable.
//   Vec     = a shopping list. Items are in order, found by position (starting at 0).
//   HashMap = a phone book. Items are key-value pairs, found by their key.

// HashMap is not available by default, so we fetch it from Rust's standard library.
use std::collections::HashMap;

fn main() {
    // ---------- Part 1: Vec, the shopping list ----------

    // A Vec is a growable list of things that are all the same type.
    // `vec!` makes one with starting items. The `!` marks it as a macro,
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

    // ---------- Part 2: Vec practice, test scores ----------

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

    // ---------- Part 3: HashMap, the phone book ----------

    // A HashMap stores key-value pairs. The key identifies the value,
    // like a name in a phone book identifies a number.
    // `::` means "belonging to": we ask HashMap to make a new, empty one.
    let mut book = HashMap::new();
    book.insert("Ada", "0801");
    book.insert("Tunde", "0802");
    book.insert("Ngozi", "0803");

    // book["Ada"] hands the book a key and gets the value back.
    // "Ada: " is just text we typed, only the lookup comes from the book.
    println!("Ada: {}", book["Ada"]);
    println!("contacts: {}", book.len());

    // Each key appears once, so inserting "Ada" again replaces her number.
    // The count stays the same.
    book.insert("Ada", "0999");
    println!("Ada now: {}", book["Ada"]);

    // Looping gives you each key-value pair. The order is not guaranteed
    // and can change between runs, because a HashMap has no positions.
    for (name, number) in &book {
        println!("{} -> {}", name, number);
    }

    // Try it: ask for a key that was never inserted, like book["Zed"].
    // The book is filled while the program runs, so Rust can't catch this
    // while compiling. It panics at runtime with "no entry found for key".
    // (A gentler way to handle a missing key comes in Lesson 12.)

    // ---------- Build: mini contact list ----------

    // These numbers are made up on purpose, since this repo is public.
    let mut contacts = HashMap::new();
    contacts.insert("Destiny", "0800000001");
    contacts.insert("Dominion", "0800000002");
    contacts.insert("Jessica", "0800000003");

    // Look up one contact by name
    println!("Destiny's number is {}", contacts["Destiny"]);

    // Change a number by inserting the same key again
    contacts.insert("Destiny", "0800000004");
    println!("Destiny's new number is {}", contacts["Destiny"]);

    // Print every contact: `i` is the name (key), `n` is the number (value)
    for (i, n) in &contacts {
        println!("{} - {}", i, n);
    }

    println!("Number of contacts is {}", contacts.len());
}