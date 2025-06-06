// define a struct's fields with types
struct User {
    active: bool,
    // NOTE: using String instead of &str string slice deliberately.
    // We want each struct instance to own all of its data and for that data
    // to be valid for as long as the entire struct is valid.
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
    // Note that the struct update syntax uses = like an assignment; this is because it moves the data.
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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // NOTE: &self is shorthand for self: &Self parameter.
    // Within an impl block, type Self is an alias for the type that the impl block is for.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        let holds_original = self.width >= other.width && self.height >= other.height;
        let holds_rotated = self.width >= other.height && self.height >= other.width;

        holds_original || holds_rotated
    }

    // Example of an associated function without a self parameter.
    // Useful for factories.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    define_and_instantiate_structs();

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rectangle 1 is {rect1:?}");
    // we can also use the dbg macro, which outputs to stderr.
    // NOTE: dbg takes ownership, unlike println which takes a reference.
    dbg!(&rect1);
    println!("The area of rectangle 1 is {} square units.", rect1.area());

    if rect1.width() {
        println!("rectangle 1 has nonzero width, which is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rect4 = Rectangle {
        width: 49,
        height: 29,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));

    let square1 = Rectangle::square(3);
    println!("square1 is {square1:?}");
}
