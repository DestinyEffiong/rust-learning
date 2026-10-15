// Milestone 2: a contact book and a todo list, in one program
//
// This joins everything from Part 2:
//   - structs (Contact and Task) to describe one thing
//   - Vec to hold many of them
//   - an enum (Command) for the choices the user can make
//   - match to react to each choice
//   - functions (parse and read) to keep repeated work in one place
//
// Think of a reception desk. The struct is the form, the Vec is the notebook
// holding all the forms, the enum is the buttons on the desk, and match is the
// receptionist who reacts to whichever button was pressed.
//
// Use made-up names and numbers when you try it, since this repo is public.

use std::io;

// One filled-in form for the contact book.
// Phone is a String, not a number, because it is a label we never do maths on
// (and a number type would drop a leading 0).
struct Contact {
    name: String,
    phone: String,
}

// One filled-in form for the todo list.
struct Task {
    title: String,
    done: bool,
    priority: i32,
}

// Every button on the desk. A match on this enum must cover all of them,
// or Rust stops with error E0004 and names the one you missed.
enum Command {
    AddContact,
    ListContacts,
    Quit,
    FindContact,
    Unknown,
    AddTask,
    ListTasks,
    DoneTask,
}

// Turns the text the user typed into one of the Command choices.
// Text goes in (&str), a Command comes out (-> Command).
// The last arm, `_`, catches any word we do not recognise.
fn parse(text: &str) -> Command {
    match text {
        "add" => Command::AddContact,
        "list" => Command::ListContacts,
        "quit" => Command::Quit,
        "find" => Command::FindContact,
        "add task" => Command::AddTask,
        "list task" => Command::ListTasks,
        "done task" => Command::DoneTask,

        _ => Command::Unknown,
    }
}

// Prints a question, reads what the user types, and hands back the trimmed text.
// We wrote these three lines many times, so they live in one function now.
// The `mut` lives in here, because this is where something writes into the box.
fn read(question: &str) -> String {
    println!("{}", question);
    let mut text = String::new();
    // `&mut text` lends the real box to read_line with permission to change it.
    // Try it: change it to `&mut text.clone()` and run it. The program still
    // compiles, but the typed text goes into a throwaway copy, so the name
    // comes out blank.
    io::stdin().read_line(&mut text).expect("could not read input");
    // Pressing Enter adds a hidden new-line (\n) to the end of the text.
    // `.trim()` removes it. No semicolon on the last line, so this value is
    // what comes out of the function.
    // Try it: remove `.trim()` and add a contact, then list it. The name keeps
    // its \n, so the colon in the list lands on the next line, and searching
    // for that name never matches.
    text.trim().to_string()
}

fn main() {
    // Two notebooks, one for each kind of form.
    // `Vec<Contact>` says what goes inside. They start empty, so Rust cannot
    // work out the type by itself. `mut` is needed because we push into them.
    let mut contacts: Vec<Contact> = Vec::new();
    let mut tasks: Vec<Task> = Vec::new();

    // The program asks for a command again and again until the user types quit.
    loop {
        let input = read("Command (add, list, quit, find, add task, list task, done task): ");

        let command = parse(input.trim());

        // This match turns a Command into an action. (The match inside parse
        // turns text into a Command. They do different jobs.)
        match command {
            Command::AddContact => {
                let name = read("Name:");

                let phone = read("Phone: ");

                let contact = Contact {
                    name: name.trim().to_string(),
                    phone: phone.trim().to_string(),
                };
                // push adds one filled-in form to the end of the notebook.
                contacts.push(contact);
                println!("Added.");
            }
            Command::ListContacts => {
                // `&contacts` means "look through the notebook, but do not take it".
                // Try it: delete the `&` and run `cargo run`.
                // Rust stops at compile time with error E0382. A for loop
                // without `&` takes the whole notebook (it moves into the loop)
                // and the loop throws it away when it finishes, so the next
                // round of the outer loop has no notebook left to use.
                for contact in &contacts {
                    println!("{} : {}", contact.name, contact.phone);
                }
            }
            Command::Quit => {
                println!("Bye");
                break;
            }
            Command::FindContact => {
                let name = read("What's the name of the contact you are looking for?");

                // A sticky note that remembers "did any form match?".
                // It is made BEFORE the loop so it survives from round to round.
                let mut found = false;

                for contact in &contacts {
                    if contact.name == name.trim() {
                        println!("phone number: {}", contact.phone);
                        found = true;
                    }
                }

                // This check is AFTER the loop, so it runs once, when every
                // form has been looked at.
                // Try it: move this `if` inside the for loop. A search that
                // succeeds still prints "not found" for every earlier form,
                // because the loop cannot know yet whether a later form matches.
                if !found {
                    println!("not found");
                }
            }
            Command::Unknown => println!("Unknown command"),
            Command::AddTask => {
                let name = read("Task Name: ");

                let priority = read("Task Priority: ");

                let status = read("Task Status: ");

                // The `: i32` on the left tells parse which kind of number to make.
                // These lines can crash if the user types the wrong thing,
                // and `expect` prints its message when that happens.
                let priority: i32 = priority.trim().parse().expect("please type a number");
                let done: bool = status.trim().parse().expect("type true or false");

                let task = Task {
                    title: name.trim().to_string(),
                    // `priority,` is shorthand for `priority: priority,`.
                    // It works because the box and the field have the same name.
                    priority,
                    done,
                };
                tasks.push(task);
                println!("Task Added.");
            }
            Command::ListTasks => {
                // The counter is made before the loop so it keeps counting.
                // Inside the loop it would reset to 1 every round.
                let mut number = 1;
                for task in &tasks {
                    println!("{}. {} ({}) done: {}", number, task.title, task.priority, task.done);
                    number += 1;
                }
            }
            Command::DoneTask => {
                let number = read("which task number have you done?");

                // A position in a Vec is always a usize (zero and up, sized to
                // your machine).
                // Try it: change usize to i32. Rust stops at compile time with
                // error E0308, "expected i32, found usize", because tasks.len()
                // gives a usize and a Vec can only be indexed by a usize.
                let number: usize = number.trim().parse().expect("please type a number");

                // The list shows numbers starting at 1, but a Vec counts from 0.
                // Check for 0 first, because 0 - 1 would crash on a usize.
                // `||` means "or", and it stops as soon as the left side is true.
                if number == 0 || number > tasks.len() {
                    println!("no task with that number");
                } else {
                    // Go straight to that form and change one box inside it.
                    // This works because `tasks` was made with `let mut`.
                    tasks[number - 1].done = true;
                    println!("Marked as done");
                }
            }
        }
    }
}