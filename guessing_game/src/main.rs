extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

mod guess;

fn main() {

  // generate random number
  let secret = rand::thread_rng().gen_range(1, guess::MAX + 1);

  println!("Guess the number! It is a positive integer in [{}, {}]", 1, guess::MAX);

  // accept user input
  loop {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Error parsing input");

    // attempt to parse numeric inputs
    let guess = match guess.trim().parse() {
      Ok(n) => guess::Guess::new(n),
        Err(e) => {
          println!("Error parsing input as numeric: {:?}", e);
          continue;
        },
    };

    println!("You guessed: {}", guess.value());

    match guess.value().cmp(&secret) {
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
