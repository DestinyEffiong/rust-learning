// Lesson 11: Lifetimes
//
// The big idea: a borrowed view can never outlive the thing it points to.
// A lifetime is how long a value stays alive, from where it is made
// to the closing brace of its owner.

// One view goes in, one view comes out.
// Rust fills in the lifetime for us, because the result can only
// come from `text`.
fn same_view(text: &str) -> &str {
    text
}

// Two views go in, one view comes out.
// Rust cannot tell which input the result comes from, so we tie them
// together with the name `'a`: the result is only valid for as long as
// BOTH inputs are.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// A struct that holds a view always needs a lifetime name.
// This says: a Book cannot outlive the text its title points at.
struct Book<'a> {
    title: &'a str,
}

fn main() {
    let x = String::from("Ada");
    let y = String::from("Tobi");

    // 1. Both owners live until the end of main, so the result is safe.
    println!("{}", longer(&x, &y));
    println!("{}", same_view(&x));

    // 2. A struct holding a view of `x`'s neighbour `name`.
    let name = String::from("Rust in Public");
    let book = Book { title: &name };
    println!("{}", book.title);

    // 3. A view cannot outlive its owner.
    // Uncomment this to see the compiler refuse (E0597):
    //
    // let result;
    // {
    //     let inner = String::from("Hi");
    //     result = longer(&x, &inner);
    // }
    // println!("{}", result);
}