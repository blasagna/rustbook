fn function_ownership() {
    let _s1 = gives_ownership(); // gives_ownership moves its return
                                 // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
                         // note: string literal is immutable

        // do stuff with s
        println!("{}", s);
    } // this scope is now over, and s is no longer valid

    // The String type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
    // You can create a String from a string literal using the from function, like so:
    let mut s2 = String::from("hello");

    // let s3 = s2; Can't do more operations on s2 after this since it would move ownership and drop s2
    let s3 = s2.clone();

    s2.push_str(", world!"); // push_str() appends a literal to a String
    println!("s2: {s2}, s3: {s3}"); // This will print `hello, world!`

    // Fixed size variables on the stack can be copied
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    function_ownership();

    // references and borrowing
    let s1 = String::from("hello");
    // We call the action of creating a reference borrowing.
    // As in real life, if a person owns something, you can borrow it from them.
    // When you’re done, you have to give it back. You don’t own it.
    let len = calculate_length(&s1);
    // We can still use s1 after calculate_length is done borrowing it
    println!("The length of '{s1}' is {len}.");

    // mutable references
    let mut s = String::from("hello");
    println!("before: {s}");
    change(&mut s);
    println!("after: {s}");

    let s = no_dangle();
    println!("{s}");

    // slices
    // Slices let you reference a __contiguous__ sequence of elements in a collection rather than the whole collection.
    // A slice is a kind of reference, so it does not have ownership.
    let s = String::from("hello world");
    // A string slice is a reference to part of a String
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}, {s}");
    let first_word = first_word(&s);
    println!("first word of {s} is {first_word}");
}
