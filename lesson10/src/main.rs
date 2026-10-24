// Lesson 10: Borrowing
//
// The big idea: you can look at a value without owning it.
// `&` is a read-only pass. `&mut` is an edit pass.
// The owner never changes, so the value is still there afterward.

// Reads a String through a read-only pass and returns its length.
fn count_letters(text: &String) -> usize {
    text.len()
}

// Changes a String through an edit pass.
fn add_exclaim(text: &mut String) {
    text.push_str("!");
}

// Adds an item to a list through an edit pass.
// `item` has no `&`, so the list takes ownership of it.
fn add_item(list: &mut Vec<String>, item: String) {
    list.push(item);
}

// Prints every item through a read-only pass.
// Looping over a `&Vec<String>` gives a borrowed view of each item.
fn show(list: &Vec<String>) {
    for item in list {
        println!("{}", item);
    }
}

fn main() {
    // 1. Borrowing to read.
    // `&word` lends a view, so `word` still works afterward.
    let word = String::from("Rustacean");
    let n = count_letters(&word);
    println!("{} has {} letters", word, n);

    // 2. Borrowing to change.
    // The variable must be `mut`, and the call must pass `&mut`.
    let mut greeting = String::from("Hello");
    add_exclaim(&mut greeting);
    println!("{}", greeting);

    // 3. A borrow lives until its last use.
    // `first` is finished before `second` is made, so there is no clash.
    let first = &mut greeting;
    first.push('?');
    let second = &mut greeting;
    second.push('?');
    println!("{}", greeting);

    // 4. The rule: many `&` readers, or exactly one `&mut` writer.
    // Uncomment these to see the compiler refuse:
    //
    // Two edit passes alive at once (E0499):
    // let a = &mut greeting;
    // let b = &mut greeting;
    // a.push('!');
    // b.push('!');
    //
    // A read pass still alive when an edit pass is made (E0502):
    // let r = &greeting;
    // let w = &mut greeting;
    // w.push('!');
    // println!("{}", r);

    // 5. Passing a list around.
    // `&mut list` lets the function add to it, `&list` only lets it look.
    let mut list: Vec<String> = Vec::new();
    add_item(&mut list, String::from("Ada"));
    add_item(&mut list, String::from("Tobi"));
    show(&list);
    println!("{} names", list.len()); // list still belongs to main
}