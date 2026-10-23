// Lesson 9: Ownership
//
// The big idea: every value has exactly one owner at a time.
// When the owner goes out of scope (reaches the closing brace of its block),
// Rust cleans up the value automatically.

// A function that takes a String, uses it, and hands it back.
// The last line has no semicolon, so `text` is the return value.
fn shout(text: String) -> String {
    println!("{}!", text);
    text
}

// A function that takes two Strings and returns the longer one.
// Both Strings move into the parameters. The loser is cleaned up
// at the closing brace of this function. The winner is returned.
fn longer(a: String, b: String) -> String {
    let bigger = a.len() > b.len();
    if bigger {
        a
    } else {
        b
    }
}

fn main() {
    // 1. Moving a String.
    // A String owns text stored on the heap, so `let b = a;` moves it.
    let a = String::from("Hello");
    let b = a;
    println!("{}", b);
    // println!("{}", a); // ERROR E0382: `a` no longer owns anything

    // 2. Copying a number.
    // Numbers are tiny and fixed in size, so they are copied, not moved.
    let x = 5;
    let y = x;
    println!("{} {}", x, y); // both still work

    // 3. Cloning a String.
    // .clone() makes a second, separate copy of the text,
    // so each variable has its own owner.
    let c = String::from("Hello");
    let d = c.clone();
    println!("{} {}", c, d);

    // 4. Passing a String into a function moves it into the parameter.
    // Giving it back through the return value lets us keep using it.
    let name = String::from("Ada");
    let name = shout(name);
    println!("{}", name);

    // 5. Two Strings go in, the longer one comes out.
    let first = String::from("Ada");
    let second = String::from("Tobi");
    let winner = longer(first, second);
    println!("{}", winner);
    // println!("{}", first); // ERROR E0382: `first` moved into `longer`
}