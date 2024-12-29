fn main() {
    // scope, mutability, and shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");


    // declare a constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours is {} seconds", THREE_HOURS_IN_SECONDS);

    // types
    let spaces = "     "; // 5 spaces
    let spaces = spaces.len();
    println!("spaces length: {}", spaces);

    // let mut spaces = "     ";
    // spaces = spaces.len();  // can't change type of mut's value

}