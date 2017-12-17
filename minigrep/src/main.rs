extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        eprintln!("usage: minigrep QUERY FILE");
        process::exit(1);
    });

    // TODO: search mulitple files using wildcards
    println!("searching for: {}", config.query);
    println!("in file: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("application error: {}", e);
        process::exit(1);
    }
}
