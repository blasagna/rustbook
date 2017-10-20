extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    const MAX: u32 = 100;

    // generate random number
    let secret = rand::thread_rng().gen_range(1, MAX + 1);

    println!("Guess the number!");

    // accept user input
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect(
            "Failed to read input",
        );

        // attempt to parse numeric inputs
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter an integer number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        };
    }

    println!("Goodbye.");
}
