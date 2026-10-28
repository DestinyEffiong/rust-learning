// Lesson 13: Traits and generics
//
// A trait is a named promise: "any type that has these functions can be used here".
// A generic function says: "give me any type, as long as it keeps that promise".

// ---------- Part 1: a trait, two types, one generic function ----------

// The trait lists what a type must be able to do, but not how.
// `&self` means "borrow the value this function is called on, read-only".
trait Speak {
    fn speak(&self) -> String;
}

// A unit struct: a type that holds no data.
struct Dog;

// A struct that holds one piece of data.
struct Robot {
    name: String,
}

// `impl Speak for Dog` means "here is how Dog keeps the Speak promise".
impl Speak for Dog {
    fn speak(&self) -> String {
        // Dog has no fields, so it never uses `self`.
        // The trait still requires `&self` in the signature.
        String::from("Woof")
    }
}

impl Speak for Robot {
    fn speak(&self) -> String {
        // `format!` works like `println!`, but returns a String instead of printing.
        // The {} is filled in by `self.name`.
        format!("Beep, I am {}", self.name)
    }
}

// `<T: Speak>` means "T is any type, as long as it implements Speak".
// This rule is called a trait bound.
fn announce<T: Speak>(thing: T) {
    // This call is allowed only because of the bound.
    // `thing.speak()` passes `thing` into `speak` as `self`, borrowed.
    println!("{}", thing.speak());
}

// ---------- Part 2: what happens when the promise is missing ----------

// Pebble never wrote `impl Speak for Pebble`, so it never kept the promise.
struct Pebble;

// Break 1: passing a type that does not implement the trait.
// The compiler refuses at the call (error E0277: trait bound not satisfied).
// announce(Pebble);

// Break 2: a generic function with no bound, but a body that calls `speak`.
// The compiler refuses inside the function (error E0599: method not found
// for type parameter `T`), because the signature promises nothing about T.
//
// fn shout<T>(thing: T) {
//     println!("{}", thing.speak());
// }
//
// The fix is to write the bound: fn shout<T: Speak>(thing: T)

// ---------- Part 3: a second trait, built from scratch ----------

trait Describe {
    fn describe(&self) -> String;
}

struct Rust;

struct TypeScript {
    name: String,
}

impl Describe for Rust {
    fn describe(&self) -> String {
        String::from("Rust is very strict with ownership and safe code")
    }
}

impl Describe for TypeScript {
    fn describe(&self) -> String {
        format!(
            "TypeScript is a {} successor, that is strict with types",
            self.name
        )
    }
}

fn print_it<T: Describe>(t: T) {
    println!("{}", t.describe());
}

fn main() {
    // Part 1
    announce(Dog);
    announce(Robot {
        name: String::from("Tobi"),
    });

    // Part 3
    print_it(Rust);
    print_it(TypeScript {
        name: String::from("JavaScript"),
    });

    // Keeps Pebble from triggering an "unused" warning.
    let _p = Pebble;
}