extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
      println!("problem parsing arguments: {}", err);
      process::exit(1);
      });

  println!("searching for: {}", config.get_query());
  println!("in file: {}", config.get_filename());

  if let Err(e) = minigrep::run(config) {
    println!("application error: {}", e);
    process::exit(1);
  }
}