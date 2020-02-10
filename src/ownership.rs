//here i got through the examples of chapter4 of the book to understand
// ownership using Strings.

// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

pub fn run() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    let x = 5;
    let y = x;
    println!("{}", s);
    take_an_integer(s); // if I pass a String to a fn, then it goes out of the block scope

    // println!("{}",s );

    println!("{}", y);

    // let _s1 = gives_ownership(); // gives_ownership moves its return
    // value into s1
    let s1 = String::from("hello");
    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn take_an_integer(s: String) {
    println!("{}", s);
}

// returning values can also pass ownership

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
