// Lesson 14: Closures and iterators
//
// A closure is a small function with no name that you can store in a variable.
// An iterator hands you the items of a collection one at a time, and you can
// attach small steps (written as closures) that work on each item.

// ---------- Part 1: closures ----------

fn closures_demo() {
    // Parameters go between pipes: |x: i32|. The body is the expression after it.
    let add_one = |x: i32| x + 1;
    println!("{}", add_one(5)); // 6

    // A closure can use a variable from outside it. This is called capturing.
    let bonus = 10;
    let add_bonus = |x: i32| x + bonus;
    println!("{}", add_bonus(5)); // 15

    // A closure that changes what it captured must itself be declared `mut`.
    let mut count = 0;
    let mut add = || count += 1;
    add();
    add();
    println!("{}", count); // 2

    // Break 1: a normal fn cannot capture variables from around it.
    // Error E0434: can't capture dynamic environment in a fn item.
    //
    // let bonus = 10;
    // fn add_bonus(x: i32) -> i32 {
    //     x + bonus
    // }

    // Break 2: forgetting `mut` on a closure that changes a captured variable.
    // Error E0596: cannot borrow `add` as mutable, as it is not declared as mutable.
    //
    // let mut count = 0;
    // let add = || count += 1;
    // add();
}

// ---------- Part 2: iterators ----------

fn iterators_demo() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // .iter() creates the iterator. It only borrows the list, so `numbers`
    // still owns its data. Each item it hands out is a view (&i32).
    // .sum() pulls every item off and adds them up. The result is one i32.
    let total: i32 = numbers.iter().sum();
    println!("{}", total); // 21

    // .map(...) runs the closure on each item and passes the results along.
    // x * 2 builds a NEW i32 from the view, so the items are now owned numbers.
    // .collect() gathers them into a new Vec. We must say which kind: Vec<i32>.
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled); // [2, 4, 6, 8, 10, 12]

    // Steps can be chained: change each item first, then add them up.
    let big_total: i32 = numbers.iter().map(|x| x * 2).sum();
    println!("{}", big_total); // 42

    // map gives back whatever the closure returns.
    // Returning x keeps the views. Returning *x follows the view to the number.
    let views: Vec<&i32> = numbers.iter().map(|x| x).collect();
    let owned: Vec<i32> = numbers.iter().map(|x| *x).collect();
    println!("{:?} {:?}", views, owned);

    // .next() hands out one item per call, inside a Some. At the end it gives None.
    let mut belt = numbers.iter();
    println!("{:?}", belt.next()); // Some(1)
    println!("{:?}", belt.next()); // Some(2)

    // Break 3: iterators are lazy. With nothing to pull the items,
    // the closure never runs. The compiler warns: "iterators are lazy
    // and do nothing unless consumed".
    //
    // numbers.iter().map(|x| {
    //     println!("closure got {}", x);
    //     x * 2
    // });

    // Break 4: collecting views into a Vec of owned numbers.
    // Error E0277: a value of type Vec<i32> cannot be built from an
    // iterator over elements of type &i32.
    //
    // let copy: Vec<i32> = numbers.iter().collect();
}

// ---------- Part 3: the build ----------

// Returns a NEW list with `amount` added to every item.
// The closure uses `amount` from outside it, so it captures it.
fn add_to_all(items: &Vec<i32>, amount: i32) -> Vec<i32> {
    items.iter().map(|x| x + amount).collect()
}

// Adds up all the items. Rust works out the type of sum() from the return type.
fn total(items: &Vec<i32>) -> i32 {
    items.iter().sum()
}

fn main() {
    closures_demo();
    iterators_demo();

    let numbers = vec![1, 2, 3, 4];
    let bumped = add_to_all(&numbers, 10);
    println!("{:?}", bumped); // [11, 12, 13, 14]
    println!("{}", total(&numbers)); // 10
    println!("{}", total(&bumped)); // 50
}