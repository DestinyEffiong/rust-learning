// Lesson 7: Structs
//
// A struct lets you bundle several related values into one thing of your own design.
// Think of a registration form at a school office:
//   - The blank printed form says which fields exist and what kind of answer each takes.
//   - A filled-in copy is one real form with real answers written in.
// In Rust, `struct` is the blank form. You can fill in as many copies as you like.
//
// If you know TypeScript, a struct is similar to an interface:
//   interface Book { title: string; pages: number; finished: boolean; }

// The blank form. Each field is written as `name: type`, followed by a comma.
// Nothing is stored here, it only describes the shape.
struct Book {
    // A struct field that holds text uses String, not &str.
    // A String owns its text. A &str only looks at text that someone else owns.
    // Try it: change `String` to `&str` here and run `cargo run`.
    // Rust stops with "error[E0106]: missing lifetime specifier".
    // It wants a written promise that the text will outlive the Book.
    // That promise is called a lifetime, and it comes later in the course.
    title: String,
    pages: u32,
    finished: bool,
}

// A second blank form, for the build of this lesson.
struct Task {
    title: String,
    done: bool,
    priority: i32,
}

fn main() {
    // Example 1: filling in a Book.
    // Write each field name, a colon, then its value. The whole thing is one
    // statement, so it ends with a semicolon after the closing brace.
    //
    // String::from("Rust Notes") means: make a new String that holds the text Rust Notes.
    // Text in quotes is a &str, and the form asked for a String, so we convert it.
    let book = Book {
        title: String::from("Rust Notes"),
        pages: 120,
        finished: false,
        // Try it: delete the `finished: false,` line above and run `cargo run`.
        // Rust stops at compile time with "error[E0063]: missing field `finished`".
        // Every field on the blank form must be filled in on every copy.
    };

    // A dot reads one field from a filled-in struct.
    println!("Title: {}", book.title);
    println!("Pages: {}", book.pages);
    println!("Finished: {}", book.finished);

    // Build: a Task struct, filled in twice.
    // Both copies come from the same blank form, with different answers.
    // Each needs its own variable name.
    let task = Task {
        title: String::from("Code Rust"),
        done: true,
        priority: 6,
    };

    println!("Title: {}", task.title);
    println!("Done: {}", task.done);
    println!("Priority: {}", task.priority);

    let task2 = Task {
        title: String::from("Read book"),
        done: true,
        priority: 5,
    };

    println!("Title: {}", task2.title);
    println!("Done: {}", task2.done);
    println!("Priority: {}", task2.priority);
}