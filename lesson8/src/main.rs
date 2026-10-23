// Lesson 8: Enums and match
//
// A struct says "this thing has ALL of these fields", like a form where every
// box gets filled in. An enum says the opposite: "this thing is EXACTLY ONE of
// these possibilities, never two at once."
//
// Think of a traffic light. It is red, or yellow, or green. Never red and
// green together, and never something outside those three.
//
// If you know TypeScript, an enum is similar to a union type:
//   type Light = "red" | "yellow" | "green";

// The list of allowed choices. Each choice is a name, separated by commas.
// Like a struct, this only describes things, nothing is stored yet.
// Try it: change `enum` to `struct` and run `cargo run`.
// Rust says `Yellow` is not found for struct `Light` (error E0599).
// A struct has fields, not choices, so `Light::Yellow` only works on an enum.
enum Light {
    Red,
    Yellow,
    Green,
}

// For the build of this lesson: the commands a program might receive.
// `Unknown` is the choice for any word we do not recognise.
enum Command {
    Open,
    Close,
    Search,
    Unknown,
}

// A function that turns text into a Command.
// Text goes in (&str), one of the Command choices comes out (-> Command).
// Try it: end this line with `;` instead of `{`.
// Rust says "free function without a body", because a function needs
// curly braces holding its body.
fn parse(text: &str) -> Command {
    // `match` looks at the text that came in and asks each arm in order:
    // "is it this one?" The first arm that fits runs.
    // Each arm is written as: pattern => value,
    match text {
        "open" => Command::Open,
        "close" => Command::Close,
        "search" => Command::Search,
        // `_` means "anything else", like the "Other" box on a form.
        // Without it, Rust could not cover every possible word.
        _ => Command::Unknown,
    }
    // No semicolon after the match: it is the value coming out of the function.
}

fn main() {
    // Example 1: pick one choice. `::` means "belonging to",
    // so Light::Yellow reads "the Yellow that belongs to Light".
    // The box `light` now holds Yellow, and only Yellow.
    let light = Light::Yellow;

    // `match` checks the value against each arm from top to bottom.
    // Red? No. Yellow? Yes, so it prints "Get ready" and stops looking.
    match light {
        Light::Red => println!("Stop"),
        Light::Yellow => println!("Get ready"),
        Light::Green => println!("Go"),
        // Try it: delete the Green arm and run `cargo run`.
        // Rust stops at compile time with "non-exhaustive patterns" (error E0004).
        // A match must cover every choice in the enum, and Rust names the one you missed.
    }

    // Example 2: one box holds exactly one choice, but a Vec can hold many.
    // Name the list with a plural word and each item with a singular word,
    // so the code reads like English: "for each light in lights".
    let lights = vec![Light::Red, Light::Yellow, Light::Green];

    for light in lights {
        // Match on the single item the loop is holding, not on the whole list.
        // Try it: write `match lights` instead.
        // Rust says "expected `Vec<Light>`, found `Light`" (error E0308),
        // because the arms ask about one Light but you gave it a whole list.
        match light {
            Light::Red => println!("Stop"),
            Light::Yellow => println!("Get ready"),
            Light::Green => println!("Go"),
        }
    }

    // Build: a command parser, in two steps.
    // Step 1: parse turns the text into a Command.
    // "delete" is not "open", "close" or "search", so the `_` arm catches it
    // and the box holds Command::Unknown.
    let command = parse("delete");

    // Step 2: a second match turns the Command into a message.
    // It needs all four arms, or Rust gives the E0004 error again.
    // Try it: change "delete" to "open" and it prints "Opening" instead.
    match command {
        Command::Open => println!("Opening"),
        Command::Close => println!("Closing"),
        Command::Search => println!("Searching"),
        Command::Unknown => println!("Unknown command"),
    }
}