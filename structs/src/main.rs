// define a struct's fields with types
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // uses field init shorthand because the username and email parameters have the same name as struct fields
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple structs without named fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs without any fields
struct AlwaysEqual;

fn define_and_instantiate_structs() {
    // create an instance of the struct with concrete values for each field
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!(
        "user active: {}, username: {}, email: {}, sign in count: {} ",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    let mut user2 = User {
        active: true,
        username: String::from("someusername456"),
        email: String::from("someoneelse@example.com"),
        sign_in_count: 2,
    };
    user2.username = String::from("anotherusername");
    println!("modified username: {}", user2.username);

    let user3 = build_user(
        String::from("thirdemail@example.com"),
        String::from("someonethree"),
    );
    println!("3's email: {}", user3.email);

    // struct update syntax
    // Note that this moves the data from user1 to user4; it does not copy it.
    let user4 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // println!("1's email: {} and username: {}", user1.email, user1.username);
    println!("1's email: {}", user1.email);
    println!(
        "4's email: {} and username: {}",
        user4.email, user4.username
    );

    // tuple structs
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
    println!("white: ({}, {}, {})", white.0, white.1, white.2);
    println!("origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself
    let _subject = AlwaysEqual;
}

fn main() {
    define_and_instantiate_structs();

    // continue with section An Example Program Using Structs
}
