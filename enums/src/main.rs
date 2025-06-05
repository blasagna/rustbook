// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }

// The code above can be made more concise in rust.

/*
We attach data to each variant of the enum directly, so there is no need for an extra struct. Here, it’s also easier to see another detail of how enums work: the name of each enum variant that we define also becomes a function that constructs an instance of the enum. That is, IpAddr::V6() is a function call that takes a String argument and returns an instance of the IpAddr type. We automatically get this constructor function defined as a result of defining the enum.
*/
enum IpAddr {
    // V4(String),
    // each enum variant can have different types and amounts of associated data
    V4(u8, u8, u8, u8),
    V6(String),
}
impl std::fmt::Display for IpAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpAddr::V4(a, b, c, d) => write!(f, "{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(addr) => write!(f, "{}", addr),
        }
    }
}
// Note that the stdlib has a definition for ip addresses, so this is just an example.
// std::net::IpAddr

fn ip_addr_example() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("Home IP: {}", home);
    println!("Loopback IP: {}", loopback);
}

// Enums can also be used to define a type that can be one of several different variants, each with its own data.
// If we used different structs, each of which has its own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum, which is a single type.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Quit => write!(f, "Quit"),
            Message::Move { x, y } => write!(f, "Move to ({}, {})", x, y),
            Message::Write(text) => write!(f, "Write: {}", text),
            Message::ChangeColor(r, g, b) => write!(f, "Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

// We are also able to define methods on enums.
impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Message called: {}", self);
    }
}

fn message_example() {
    let m1 = Message::Write(String::from("hello"));
    m1.call();
    let m2 = Message::Move { x: 10, y: 20 };
    m2.call();
    let m3 = Message::ChangeColor(255, 0, 0);
    m3.call();
    let m4 = Message::Quit;
    m4.call();
    println!("Messages processed.");
}

// The Option enum.
// Rust does not have nulls, but it does have an enum called Option that can be used to express the concept of a value that can be something or nothing.
// NOTE: Option is defined in the standard library at std::option::Option
// enum Option<T> {
//     Some(T),
//     None,
// }
use std::option::Option;

fn option_example() {
    let some_number = Option::Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = Option::None;

    println!(
        "Some number: {}",
        some_number.expect("some number should be present!")
    );
    println!(
        "Some char: {}",
        some_char.expect("some char should be present!")
    );
    println!("absent number is none: {}", absent_number.is_none());
}

// The match control flow construct.
// continue with chapter 6.2

fn main() {
    ip_addr_example();
    message_example();
    option_example();
}
