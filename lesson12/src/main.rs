// Lesson 12: Errors (Option and Result)
//
// The big idea: when something might not exist or might fail,
// Rust makes that part of the answer itself.
// Option: Some(value) or None.
// Result: Ok(value) or Err(reason).

// Turns text into a number and describes it.
// `parse` hands back a Result, so we open it with `match`.
// Both arms give back a String, so the function returns a String.
fn describe(text: &str) -> String {
    match text.parse::<i32>() {
        Ok(n) => format!("double is {}", n * 2),
        Err(_) => String::from("not a number"),
    }
}

fn main() {
    // 1. Option: `.get` hands back Some or None.
    let list = vec![10, 20, 30];
    let found = list.get(1);
    let missing = list.get(5);

    // `{:?}` prints the box with its label showing.
    println!("{:?}", found);
    println!("{:?}", missing);

    // Open the box with `match`. Both labels must be handled.
    match found {
        Some(n) => println!("got {}", n),
        None => println!("nothing there"),
    }
    match missing {
        Some(n) => println!("got {}", n),
        None => println!("nothing there"),
    }

    // 2. Result: `.parse` hands back Ok or Err, and Err says why.
    let good = "42".parse::<i32>();
    let bad = "abc".parse::<i32>();
    println!("{:?}", good);
    println!("{:?}", bad);

    // 3. Our own function that opens the Result for us.
    println!("{}", describe("21"));
    println!("{}", describe("abc"));

    // 4. Things the compiler refuses.
    // Uncomment these to see the errors:
    //
    // You cannot use the box as if it were the number (E0369):
    // let c = found + 1;
    //
    // A match must cover every label (E0004):
    // match found {
    //     Some(n) => println!("got {}", n),
    // }
}