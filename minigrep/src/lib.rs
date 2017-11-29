use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
query: String,
         filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    Ok(Config { query, filename })
  }

  pub fn get_query(&self) -> &str {
    &self.query
  }

  pub fn get_filename(&self) -> &str {
    &self.filename
  }
}

pub fn run(config: Config) -> Result<(), Box<Error>>{
  let mut f = File::open(config.filename)?;

  let mut contents = String::new();
  f.read_to_string(&mut contents)?;

  println!("with text:\n{}", contents);

  Ok(())
}

// FIXME: add tests
